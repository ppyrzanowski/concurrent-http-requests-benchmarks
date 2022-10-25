use std::thread;

fn main() {
    println!("{}", thread::available_parallelism().unwrap().get());
}
