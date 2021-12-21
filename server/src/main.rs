pub mod config;

fn main() {
    let config = config::Config::default();
    config.init_logger();
    println!("Hello, world!");
}
