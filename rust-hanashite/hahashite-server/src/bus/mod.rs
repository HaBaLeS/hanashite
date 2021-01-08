#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::broadcast::*;
use tokio::sync::broadcast::error::{RecvError, SendError};
use tokio::sync::Mutex;


pub struct BusMessage<T, U> {
    pub predicate: Arc<BusPredicate<T, U>>,
    pub msg: T,
}

pub trait MessagePredicate<T, U> {
    fn relevant(&self, message: &BusMessage<T, U>, context: &U) -> bool;
}

pub type BusPredicate<T, U> = dyn MessagePredicate<T, U> + Send + Sync;

pub struct BusEndpoint<T, U> {
    receiver: Receiver<Arc<BusMessage<T, U>>>,
    sender: Sender<Arc<BusMessage<T, U>>>,
    context: U,
}

pub struct Bus<T, U> {
    sender: Sender<Arc<BusMessage<T, U>>>,
}

impl<T, U> Bus<T, U> {
    pub fn new() -> Self {
        let (sender, _) = channel(16384);
        Bus {
            sender
        }
    }
}

impl<T, U> Bus<T, U>
    where U: Default {
    pub fn create_endpoint(&self) -> BusEndpoint<T, U> {
        BusEndpoint {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
            context: Default::default(),
        }
    }

    pub fn send(&mut self, message: Arc<BusMessage<T, U>>) -> Result<(), SendError<Arc<BusMessage<T, U>>>> {
        self.sender.send(message)?;
        Ok(())
    }
}

impl<T, U: Default> BusEndpoint<T, U> {
    pub fn mut_context(&mut self) -> &mut U {
        return &mut self.context;
    }

    pub fn context(&self) -> &U {
        return &self.context;
    }

    pub async fn recv(&mut self, predicate: &BusPredicate<T, U>) -> Result<Arc<BusMessage<T, U>>, RecvError> {
        loop {
            let message = self.receiver.recv().await?;
            if message.predicate.relevant(message.as_ref(), self.context()) &&
                predicate.relevant(message.as_ref(), self.context()) {
                return Ok(message);
            }
        }
    }

    pub fn send(&mut self, message: Arc<BusMessage<T, U>>) -> Result<(), SendError<Arc<BusMessage<T, U>>>> {
        self.sender.send(message)?;
        Ok(())
    }
}

impl<T, U: Clone> Clone for BusEndpoint<T, U> {
    fn clone(&self) -> Self {
        BusEndpoint {
            sender: self.sender.clone(),
            receiver: self.sender.subscribe(),
            context: self.context.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_me() {
        enum Test {
            A {
                a: i32,
                b: i32,
            }
        }
        let b = Test::A { a: 1, b: 2 };
        let Test::A { a, .. } = b;
        println!("{}", a);
    }
}

