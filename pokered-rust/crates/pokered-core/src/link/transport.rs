use super::protocol::NetworkMessage;
use std::sync::mpsc;

#[derive(Debug)]
pub enum TransportError {
    Disconnected,
    Timeout,
    SerializationError(String),
    IoError(String),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::Disconnected => write!(f, "peer disconnected"),
            TransportError::Timeout => write!(f, "operation timed out"),
            TransportError::SerializationError(e) => write!(f, "serialization error: {}", e),
            TransportError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

pub trait NetworkTransport {
    fn send(&mut self, msg: NetworkMessage) -> Result<(), TransportError>;
    fn recv(&mut self) -> Result<NetworkMessage, TransportError>;
    fn try_recv(&mut self) -> Result<Option<NetworkMessage>, TransportError>;
}

pub struct ChannelTransport {
    tx: mpsc::Sender<NetworkMessage>,
    rx: mpsc::Receiver<NetworkMessage>,
}

impl ChannelTransport {
    pub fn new_pair() -> (Self, Self) {
        let (tx_a, rx_b) = mpsc::channel();
        let (tx_b, rx_a) = mpsc::channel();
        (
            ChannelTransport { tx: tx_a, rx: rx_a },
            ChannelTransport { tx: tx_b, rx: rx_b },
        )
    }
}

impl NetworkTransport for ChannelTransport {
    fn send(&mut self, msg: NetworkMessage) -> Result<(), TransportError> {
        self.tx.send(msg).map_err(|_| TransportError::Disconnected)
    }

    fn recv(&mut self) -> Result<NetworkMessage, TransportError> {
        self.rx.recv().map_err(|_| TransportError::Disconnected)
    }

    fn try_recv(&mut self) -> Result<Option<NetworkMessage>, TransportError> {
        match self.rx.try_recv() {
            Ok(msg) => Ok(Some(msg)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(TransportError::Disconnected),
        }
    }
}
