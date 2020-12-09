use crate::server::*;
use std::sync::atomic::Ordering;
use rand::{RngCore, thread_rng};
use sodiumoxide::crypto::sign::{verify_detached, Signature};

pub trait AuthServer {

    fn auth_request(&self, connection_id: &Uuid, user_name: &str, public_key: &PublicKey) -> Result<AuthStatus, Error>;
    fn challenge_result_request(&self, connection_id: &Uuid, signature: &Vec<u8>) -> Result<AuthStatus, Error>;
}


#[derive(PartialEq, Eq, Debug, Clone)]
#[allow(dead_code)]
pub enum AuthStatus {
    KeyChallenge(Vec<u8>),
    Success(u32, String),
    Failed,
    Unknown,
}

impl AuthServer for ServerStruct {

    fn auth_request(&self, connection_id: &Uuid, user_name: &str, public_key: &PublicKey) -> Result<AuthStatus, Error> {
        let mut connections = self.connections.lock().unwrap();
        let mut connection = if let Some(con) = connections.get_mut(connection_id) {
            con
        } else {
            return Err(Error::InternalError("Illegal Connection used.".to_string()));
        };
        if connection.state != ConnectionState::Connected {
            connection.state = ConnectionState::Defunct;
            return Err(Error::ProtocolError("Illegal State".to_string()));
        }
        let mut users = self.users.lock().unwrap();
        let user_id = if let Some(user) = users.values()
            .find(|user| &user.public_key == public_key) {
            user.id
        } else {
            let user_id = self.next_user_id.fetch_add(1, Ordering::Relaxed);
            users.insert(user_id, User {
                id: user_id,
                public_key: public_key.clone(),
            });
            user_id
        };
        let mut challenge = vec![0; 16];
        thread_rng().fill_bytes(&mut challenge);
        connection.user_id = user_id;
        connection.name = user_name.to_string();
        connection.state = ConnectionState::Challenged(challenge.clone());
        Ok(AuthStatus::KeyChallenge(challenge))
    }

    fn challenge_result_request(&self, connection_id: &Uuid, signature: &Vec<u8>) -> Result<AuthStatus, Error> {
        let mut connections = self.connections.lock().unwrap();
        let mut connection = if let Some(con) = connections.get_mut(connection_id) {
            con
        } else {
            return Err(Error::InternalError("Illegal Connection used.".to_string()));
        };
        let challenge = if let ConnectionState::Challenged(c) = &connection.state {
            c
        } else {
            connection.state = ConnectionState::Defunct;
            return Err(Error::InternalError("Connection State wrong.".to_string()));
        };
        let users = self.users.lock().unwrap();
        let user = if let Some(usr) = users.get(&connection.user_id) {
            usr
        } else {
            connection.state = ConnectionState::Defunct;
            return Err(Error::InternalError("Illegal UserId used.".to_string()));
        };

        if let Some(s) = Signature::from_slice(signature) {
            if verify_detached(&s, challenge, &user.public_key) {
                connection.state = ConnectionState::Authenticated;
                return Ok(AuthStatus::Success(user.id, connection.name.clone()));
            }
        }
        connection.state = ConnectionState::Defunct;
        Err(Error::ProtocolError("Illegal Signature
            ".to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::server::test::*;
    use assert_matches::assert_matches;
    use sodiumoxide::crypto::sign::{sign_detached};

    #[test]
    fn test_auth() {
        let data = setup_test();
        let result = data.server.auth_request(&data.connections[0],
                                              "testuser",
                                              &data.public_keys[0]).unwrap();
        if let AuthStatus::KeyChallenge(c) = result {
            let connections = data.server.connections.lock().unwrap();
            let con = connections.get(&data.connections[0]).unwrap();
            assert_matches!(&con.state,  ConnectionState::Challenged(chall) if &c == chall);
            assert_eq!(2, data.server.next_user_id.load(Ordering::Relaxed));
            assert_eq!(1, con.user_id);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_new_auth() {
        let data = setup_test();
        assert_eq!(2, data.server.next_user_id.load(Ordering::Relaxed));
        let result = data.server.auth_request(&data.connections[0],
                                              "testuser",
                                              &data.public_keys[1]).unwrap();
        if let AuthStatus::KeyChallenge(c) = result {
            let connections = data.server.connections.lock().unwrap();
            let con = connections.get(&data.connections[0]).unwrap();
            assert_matches!(&con.state,  ConnectionState::Challenged(chall) if &c == chall);
            assert_eq!(3, data.server.next_user_id.load(Ordering::Relaxed));
            assert_eq!(2, con.user_id);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_challenge() {
        let data = setup_test();
        let signature = sign_detached(data.challenge.as_slice(), &data.secret_keys[0]);
        let result = data.server.challenge_result_request(&data.connections[2],
                                                          &Vec::from(signature.0)).unwrap();
        if let AuthStatus::Success(user_id, user_name) = result {
            assert_eq!(1, user_id);
            assert_eq!("testuser2".to_string(), user_name);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_faled_challenge() {
        let data = setup_test();
        let signature = sign_detached(data.challenge.as_slice(), &data.secret_keys[1]);
        let result = data.server.challenge_result_request(&data.connections[2],
                                                          &Vec::from(signature.0)).unwrap_err();
        assert_matches!(result, Error::ProtocolError(_));
    }




}