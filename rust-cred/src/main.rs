use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct Cred {
    username: String,
    password: String,
}

fn test_ssh(host: &str, port: u16, user: &str, pass: &str) -> bool {
    match std::net::TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(_) => {
            match ssh2::Session::new() {
                Ok(sess) => {
                    if let Ok(stream) = std::net::TcpStream::connect(format!("{}:{}", host, port)) {
                        if sess.set_tcp_stream(stream).is_ok() {
                            if sess.handshake().is_ok() {
                                return sess.userauth_password(user, pass).is_ok();
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => return false,
    }
    false
}

fn test_http(host: &str, port: u16, user: &str, pass: &str) -> bool {
    let url = format!("http://{}:{}", host, port);
    match std::net::TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: {} <host> <port> <ssh|http> <wordlist>", args[0]);
        return;
    }

    let host = args[1].clone();
    let port: u16 = args[2].parse().unwrap_or(22);
    let proto = args[3].clone();

    let file = match File::open(&args[4]) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Cannot open wordlist");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut creds = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                creds.push(Cred {
                    username: parts[0].to_string(),
                    password: parts[1].to_string(),
                });
            }
        }
    }

    let found = Arc::new(AtomicUsize::new(0));
    println!("🔑 Credential Tester Started");
    println!("Target: {}:{}\nProtocol: {}\nCredentials: {}\n", host, port, proto, creds.len());

    for cred in creds {
        let success = match proto.as_str() {
            "ssh" => test_ssh(&host, port, &cred.username, &cred.password),
            "http" => test_http(&host, port, &cred.username, &cred.password),
            _ => false,
        };

        if success {
            println!("✅ FOUND: {}:{}", cred.username, cred.password);
            found.fetch_add(1, Ordering::Relaxed);
        } else {
            println!("❌ {}:{}", cred.username, cred.password);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    println!("\n📊 Results: {} found", found.load(Ordering::Relaxed));
}