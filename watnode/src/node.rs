use crate::types::{Peer, PeerMetaData, PeerStatus};
use std::{collections::HashMap, net::SocketAddr};

pub struct Node {
    address: SocketAddr,
    is_gateway: bool,
    peers: HashMap<SocketAddr, PeerMetaData>,
}

impl Node {
    pub fn new(address: SocketAddr, is_gateway: bool) -> Self {
        Self {
            address,
            is_gateway,
            peers: HashMap::new(),
        }
    }

    /// Merges a received peer list into the node's internal map, avoiding duplicates and updating metadata
    fn merge_peer_lists(&mut self, recived: Vec<Peer>) {
        for peer_received in recived {
            // Skip self
            if peer_received.address == self.address {
                continue;
            }
            self.peers
                .entry(peer_received.address)
                .and_modify(|existing| {
                    // Update gateway flag and last-seen timestamp
                    existing.is_gateway |= peer_received.is_gateway;
                    existing.last_seen_by_network = Self::latest(
                        existing.last_seen_by_network,
                        peer_received.last_seen_timestamp,
                    );
                })
                .or_insert_with(|| PeerMetaData {
                    is_gateway: peer_received.is_gateway,
                    last_seen_by_network: peer_received.last_seen_timestamp,
                    last_tried: None,
                    status: PeerStatus::Disconnected,
                });
        }
    }

    /// Returns the current peer list as a Vec<Peer>
    pub fn get_peer_list(&self) -> Vec<Peer> {
        self.peers
            .iter()
            .map(|(&address, meta)| Peer {
                address,
                is_gateway: meta.is_gateway,
                last_seen_timestamp: meta.last_seen_by_network,
            })
            .collect()
    }

    /// Utility to pick the most recent Instant
    fn latest(a: Option<u64>, b: Option<u64>) -> Option<u64> {
        match (a, b) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        }
    }
}
