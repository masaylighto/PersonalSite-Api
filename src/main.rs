use std::env;
mod web_server;
fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    println!("{:?} ",env::current_dir().unwrap());
    // we start the server
    web_server::start_the_server()
}
