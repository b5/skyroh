use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use iroh_net::NodeId;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeeringState {
    // DID of the user running this node
    pub host_did: String,
    // vec of (DID, NodeId)
    pub peers: Vec<(String, NodeId)>,
}

const PEERING_FILENAME: &str = "peers.json";

#[derive(Debug, Clone)]
pub struct Peering {
    file_path: PathBuf,
    state: Arc<RwLock<PeeringState>>,
}

impl Peering {
    pub async fn open_or_create(host_did: &str, base_path: impl Into<PathBuf>) -> Result<Self> {
        let path = base_path.into().join(PEERING_FILENAME);
        if path.exists() {
            return Self::open(host_did, &path).await;
        }
        Self::create(host_did, &path).await
    }

    async fn open(host_did: &str, file_path: &PathBuf) -> Result<Self> {
        let state = Self::read_from_file(host_did, file_path).await?;
        Ok(Self {
            file_path: file_path.into(),
            state: Arc::new(RwLock::new(state)),
        })
    }

    async fn create(host_did: &str, file_path: &PathBuf) -> Result<Self> {
        // create the space
        let peering = Self {
            file_path: file_path.into(),
            state: Arc::new(RwLock::new(PeeringState {
                host_did: host_did.to_string(),
                peers: vec![],
            })),
        };
        peering.write_to_file().await?;
        Ok(peering)
    }

    async fn read_from_file(host_did: &str, file_path: &PathBuf) -> Result<PeeringState> {
        if !file_path.exists() {
            return Ok(PeeringState {
                host_did: host_did.to_string(),
                peers: vec![],
            });
        }
        let file = tokio::fs::read(&file_path).await?;
        let state = serde_json::from_slice(&file)?;
        debug!("reading peering file: {:?}", file_path);
        Ok(state)
    }

    async fn write_to_file(&self) -> Result<()> {
        let inner = self.state.read().await.clone();
        let file = serde_json::to_vec(&inner)?;
        debug!("writing peering file: {:?}", self.file_path);
        tokio::fs::write(&self.file_path, file).await?;
        Ok(())
    }
}
