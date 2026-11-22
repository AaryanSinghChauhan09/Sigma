//! Channel for message passing between threads
//! Follows principles: Safe Concurrency, Message Passing, Actor Model

use std::sync::mpsc;

pub struct ChannelOps<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
}

impl<T> ChannelOps<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self { sender, receiver }
    }
    pub fn send(&self, msg: T) -> Result<(), mpsc::SendError<T>> { self.sender.send(msg) }
    pub fn recv(&self) -> Result<T, mpsc::RecvError> { self.receiver.recv() }
}

#[cfg(test)] mod tests { use super::*; #[test] fn test_new() { let _ = ChannelOps::<i32>::new(); } }
