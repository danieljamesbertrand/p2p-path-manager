//! # P2P Path Manager
//!
//! Intelligent P2P path management library with relay fallback and NAT hole punching
//! using libp2p, QUIC, and DCUtR.
//!
//! ## Architecture
//!
//! This library implements a boxed heuristic model with five main components:
//!
//! 1. **Discovery & Relay Setup** - Establishes initial relayed connections
//! 2. **Heuristics** - Decides when to attempt hole punching based on network conditions
//! 3. **Punch Orchestrator** - Coordinates DCUtR hole punching attempts
//! 4. **Path Selection** - Manages switching between relay and direct paths
//! 5. **Metrics & Learning** - Tracks outcomes to improve future decisions

pub mod discovery;
pub mod heuristics;
pub mod punch;
pub mod selection;
pub mod metrics;

use libp2p::PeerId;
use std::fmt;

/// Represents an active connection path to a peer
#[derive(Debug, Clone)]
pub enum ActivePath {
    /// Connection is relayed through an intermediary
    Relay(RelayHandle),
    /// Direct connection to peer
    Direct(DirectHandle),
}

/// Handle for a relayed connection
#[derive(Debug, Clone)]
pub struct RelayHandle {
    pub peer_id: PeerId,
    pub relay_peer_id: PeerId,
    pub rtt_ms: u64,
}

/// Handle for a direct connection
#[derive(Debug, Clone)}
pub struct DirectHandle {
    pub peer_id: PeerId,
    pub rtt_ms: u64,
    pub endpoint: String,
}

/// Configuration for the PathManager
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum RTT (ms) before attempting to punch
    pub max_relay_rtt_ms: u64,
    /// Minimum success rate to keep attempting punches
    pub min_punch_success_rate: f64,
    /// Backoff multiplier after failed punch attempts
    pub punch_backoff_multiplier: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_relay_rtt_ms: 200,
            min_punch_success_rate: 0.3,
            punch_backoff_multiplier: 2.0,
        }
    }
}

/// Main path manager for handling P2P connections
pub struct PathManager {
    config: Config,
}

impl PathManager {
    /// Create a new PathManager with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Create a new PathManager with default configuration
    pub fn with_defaults() -> Self {
        Self::new(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config::default();
        assert_eq!(config.max_relay_rtt_ms, 200);
        assert!(config.min_punch_success_rate > 0.0);
    }

    #[test]
    fn test_path_manager_creation() {
        let manager = PathManager::with_defaults();
        assert_eq!(manager.config.max_relay_rtt_ms, 200);
    }
}
