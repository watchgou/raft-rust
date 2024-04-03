use serde::Deserialize;

pub mod config;
pub mod filter;
pub mod raft_log;

#[derive(Default, Deserialize, Debug)]
pub struct RaftConfig {
    pub master: Option<String>,
    pub slaver: Option<Vec<String>>,
    pub raft_log_path: Option<String>,
}
