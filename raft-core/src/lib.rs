mod code;
mod heartbeat;
mod protos;
mod raft;
mod server;

use std::{thread, time::Duration};

use derive_builder::Builder;
use raft::{Event, StateMachine};

use raft_common::{
    config::config_util::{ParseConfig, C},
    filter,
    raft_log::log::LogModule,
};
use rand::Rng;
use serde::Deserialize;
use tokio::sync::mpsc::{self, Receiver};

#[derive(Default, Deserialize, Builder, Debug, Clone)]
pub(crate) struct RaftConfig {
    pub cluster: Option<Vec<String>>,
    pub host_names: Option<String>,
    pub raft_log_path: Option<String>,
    pub out_time: Option<u64>,
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

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut yaml = String::new();
    let mut conf: RaftConfig = C::parse(
        "/Users/jon/workspace/rust/raft-rust/raft_config.yaml",
        &mut yaml,
    );
    let log = LogModule::new(&conf.raft_log_path);
    let millis = rand::thread_rng().gen_range(2000..=5000) as u64;
    conf.out_time(millis);
    // 初始化 过滤器
    filter::FilterManager::new();
    let (tx, rx) = mpsc::channel::<String>(5);

    let task = tokio::spawn(async_task(rx));
    let state_task = tokio::spawn(async move {
        let mut state = StateMachine::new(log, &conf, tx);
        state.handle_event(Event::Init).await;
    });
    let _ = tokio::join!(task, state_task);
    Ok(())
}

async fn async_task(mut rx: Receiver<String>) {
    let data = rx.recv().await;
    if let Some(data) = data {
        loop {
            log::info!("state {:?}", data);
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
