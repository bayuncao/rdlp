// tests/server_integration_tests.rs

use std::process::Command;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

#[test]
fn test_run_server_command_integration() {
    let port = 8080;
    let mut child = Command::new("./target/debug/rdlp")
        .args(["server", "--port", &port.to_string()])
        .spawn()
        .expect("failed to start test server");

    // Allow some time for the server to start
    thread::sleep(Duration::from_secs(2));

    // Attempt to connect to the server
    match TcpStream::connect(format!("127.0.0.1:{}", port)) {
        Ok(_) => println!("Successfully connected to server on port {}", port),
        Err(e) => panic!("Failed to connect to server: {}", e),
    }

    // Clean up: Kill the server process after the test
    child.kill().expect("server process failed to exit");
}
