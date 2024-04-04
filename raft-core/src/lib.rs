use raft_common::*;

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

pub fn start() {
    // 初始化 过滤器
    filter::FilterChain::init();
}
