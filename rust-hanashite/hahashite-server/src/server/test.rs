#![cfg(test)]
use tokio::sync::{mpsc, broadcast};
use std::sync::Arc;
use crate::server::{ServerStruct, ConnectionState, Connection, User};
use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::{PublicKey, SecretKey};
use uuid::Uuid;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::str::FromStr;
use rand::{RngCore, thread_rng};



pub struct TestData {
    pub server: Arc<ServerStruct>,
    pub public_keys: Vec<PublicKey>,
    pub secret_keys: Vec<SecretKey>,
    pub connections: Vec<Uuid>,
    pub challenge: Vec<u8>,
}


pub fn setup_test() -> TestData {
    let mut result = TestData {
        server: ServerStruct::init(crate::configuration::default()),
        public_keys: vec![],
        secret_keys: vec![],
        connections: vec![],
        challenge: vec![],
    };
    {
        let (termination_sender, _) = broadcast::channel(2);
        let mut connections = result.server.connections.lock().unwrap();
        let connection_id = Uuid::new_v4();
        let (sender, _) = mpsc::channel(100);
        result.connections.push(connection_id.clone());
        connections.insert(connection_id, Connection {
            state: ConnectionState::Connected,
            user_id: 0,
            name: "
            ".to_string(),
            udp_addr: None,
            peer_addr: SocketAddr::from_str("127.0.0.1:420").unwrap(),
            sender,
            termination_sender: termination_sender.clone(),
        });
        let connection_id = Uuid::new_v4();
        let (sender, _) = mpsc::channel(100);
        result.connections.push(connection_id.clone());
        connections.insert(connection_id, Connection {
            state: ConnectionState::Authenticated,
            user_id: 1,
            name: "testuser2".to_string(),
            udp_addr: None,
            peer_addr: SocketAddr::from_str("127.0.0.1:420").unwrap(),
            sender,
            termination_sender: termination_sender.clone(),
        });
        let connection_id = Uuid::new_v4();
        let (sender, _) = mpsc::channel(100);
        result.connections.push(connection_id.clone());
        result.challenge = vec![0; 16];
        thread_rng().fill_bytes(result.challenge.as_mut_slice());
        connections.insert(connection_id, Connection {
            state: ConnectionState::Challenged(result.challenge.clone()),
            user_id: 1,
            name: "testuser2".to_string(),
            udp_addr: None,
            peer_addr: SocketAddr::from_str("127.0.0.1:420").unwrap(),
            sender,
            termination_sender,
        });
        let mut users = result.server.users.lock().unwrap();

        result.server.next_user_id.fetch_add(1, Ordering::Relaxed);
        let (pk, sk) = sign::gen_keypair();
        users.insert(1, User {
            id: 1,
            public_key: pk.clone(),
        });
        result.public_keys.push(pk);
        result.secret_keys.push(sk);
        let (pk, sk) = sign::gen_keypair();
        result.public_keys.push(pk);
        result.secret_keys.push(sk);
    }
    result
}