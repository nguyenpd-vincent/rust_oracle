use env_logger::Env;
use log::info;

pub fn init_logging() {
    std::env::set_var("RUST_BACKTRACE", "full");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Logger initialized");
}