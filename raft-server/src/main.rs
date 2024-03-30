use raft_code::protos;

fn main() {
    let he = protos::hello::HotWordRequest::default();
    println!("{:?}", he);
}
