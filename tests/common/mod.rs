use std::net::TcpListener;

use newsletter_api::run;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    let _handle = tokio::spawn(server);
    let address = format!("http://127.0.0.1:{}", port);
    address
}
