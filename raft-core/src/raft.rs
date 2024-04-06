use crate::{server::rpc_server::vote_server, RaftConfig};

use raft_common::raft_log::log::LogModule;

use tokio::sync::mpsc::Sender;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum State {
    Follower,

    // 领导者
    Leader,

    // 候选者
    Candidate,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Event {
    Init,
    Election,
    Success,
}

pub(crate) struct StateMachine<'a> {
    state: State,
    log: LogModule,
    pub(crate) conf: &'a RaftConfig,
    pub(crate) leader_node: Option<String>,
    tx: Sender<String>,
}

impl<'a> StateMachine<'a> {
    pub(crate) fn new(log: LogModule, conf: &'a RaftConfig, tx: Sender<String>) -> StateMachine {
        StateMachine {
            state: State::Follower,
            log,
            conf,
            leader_node: None,
            tx,
        }
    }

    pub(crate) async fn handle_event(&mut self, event: Event) {
        //self.tx.send("None".to_string()).await.unwrap();
        match (self.state, event) {
            (State::Follower, Event::Init) => {
                vote_server(&self.conf).await;
                //self.state = State::StateA;
                self.tx.send("value".to_string()).await.unwrap();
            }
            (State::Leader, Event::Success) => {
                //self.state = State::StateB;
            }
            (State::Candidate, Event::Election) => {
                // self.state = State::Final;
            }
            _ => {}
        }
    }

    pub(crate) async fn get_current_state(&self) -> &State {
        &self.state
    }

    pub(crate) async fn query_leader_node(&self) -> Option<String> {
        if let Some(leader_node) = &self.leader_node {
            Some(leader_node.clone())
        } else {
            None
        }
    }
}

pub(crate) async fn request_process() {}
