use tokio::net::TcpListener;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct Node {
    pub address: String,
    pub peers: Vec<String>,
    pub bchain: Arc<Mutex<BlockChain>>,
}
