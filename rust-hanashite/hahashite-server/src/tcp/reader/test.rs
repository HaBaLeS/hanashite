#![cfg(test)]
#![allow(dead_code)]
use tokio::time::{timeout, Duration};
use tokio::sync::mpsc::Receiver;
use crate::error::Error;

pub async fn try_recv<T>(receiver: &mut Receiver<T>) -> Result<T, Error> {
    match timeout(Duration::from_millis(10), receiver.recv()).await {
        Err(_) => Err(Error::InternalError("No Message in queue".to_string())),
        Ok(Some(t)) => Ok(t),
        Ok(None) => Err(Error::InternalError("Queue closed".to_string()))
    }
}