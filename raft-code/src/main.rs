mod protos;
use protos::hello;
fn main() {
    let hello = hello::HotWordRequest::default();
    println!("{:?}", hello);
}
