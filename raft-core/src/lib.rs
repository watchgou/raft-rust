mod code;
mod heartbeat;
mod protos;
mod server;
use raft_common::{
    config::config_util::{ParseConfig, C},
    raft_log::log::LogModule,
    *,
};
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum State {
    // 追随者
    #[default]
    Follower,

    // 领导者
    Leader,

    // 候选者
    Candidate,
}
#[derive(Default, Deserialize, Debug)]
struct RaftConfig {
    pub cluster: Option<Vec<String>>,
    pub host_names: Option<String>,
    pub raft_log_path: Option<String>,
}

pub async fn start() {
    // load configuration
    //
    let mut yaml = String::new();
    let conf: RaftConfig = C::parse(
        "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
        &mut yaml,
    );

    let _log = LogModule::new(&conf.raft_log_path);

    // 初始化 过滤器
    filter::FilterChain::init();

    let mut data = State::default();
    loop {
        match data {
            State::Follower => {
                println!("Follower");
            }
            State::Leader => {
                println!("Leader");
            }
            State::Candidate => {
                println!("Candidate");
            }
        }
        data = State::Leader;
    }
}
