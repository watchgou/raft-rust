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
    Followers,

    // 领导者
    Leader,

    // 候选者
    Candidate,
}
#[derive(Default, Deserialize, Debug)]
struct RaftConfig {
    cluster: Option<Vec<String>>,
    host_names: Option<String>,
    raft_log_path: Option<String>,
}

pub fn start() {
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
}
