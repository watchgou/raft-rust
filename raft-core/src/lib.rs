mod code;
mod heartbeat;
mod protos;
mod server;

use std::{thread, time::Duration};

use crate::server::rpc_server::vote_server;

use derive_builder::Builder;
use raft_common::{
    config::config_util::{ParseConfig, C},
    raft_log::log::LogModule,
    *,
};
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum State {
    #[default]
    Follower,

    // 领导者
    Leader,

    // 候选者
    Candidate,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    #[default]
    Start,
    Running,
    Finished,
}

#[derive(Default, Deserialize, Debug, Clone, Builder)]
struct RaftConfig {
    pub cluster: Option<Vec<String>>,
    pub host_names: Option<String>,
    pub raft_log_path: Option<String>,
    pub out_time: Option<u64>,
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    // load configuration
    //
    let mut yaml = String::new();
    let mut conf: RaftConfig = C::parse(
        "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
        &mut yaml,
    );
    let _log = LogModule::new(&conf.raft_log_path);

    // 初始化 过滤器
    filter::FilterChain::init();

    let mut rng = rand::thread_rng();
    let millis = rng.gen_range(1000..=5000) as u64;
    conf.out_time(millis);

    let state = State::default();

    state_machine(state, &conf).await;

    Ok(())
}

struct StateMachine {
    state: State,
}

async fn state_machine(state: State, conf: &RaftConfig) {
    match state {
        State::Follower => {
            let _ = vote_server(conf);
            Box::pin(state_machine(State::Candidate, &conf)).await;
        }
        State::Leader => {}
        State::Candidate => {
            log::info!("Candidate");
            thread::sleep(Duration::from_millis(10000));
        }
    }
}

impl RaftConfig {
    fn cluster(&mut self, host: String) {
        if let Some(mut cluster) = self.cluster.clone() {
            cluster.push(host);
        }
    }

    fn out_time(&mut self, millis: u64) {
        self.out_time = Some(millis);
    }
}
