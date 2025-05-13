use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Represents a known peer in the network
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peer {
    pub address: SocketAddr,
    pub is_gateway: bool,
    pub last_seen_timestamp: Option<u64>,
}

/// The peer list map: SocketAddr -> metadata
pub type PeerList = std::collections::HashMap<SocketAddr, PeerMetaData>;

/// Message types for the P2P protocol
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    SendWasm,
    ExecutionResult,
    PeerList,
}

/// Connection status of a peer
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PeerStatus {
    Connected,
    Disconnected,
    Unreachable,
    Blacklisted,
}

/// Metadata stored for each peer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerMetaData {
    pub is_gateway: bool,
    pub last_seen_by_network: Option<u64>,
    pub last_tried: Option<u64>,
    pub status: PeerStatus,
}
