use clap::Parser;
use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}
};
use reqwest::Client;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::task;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    port: String,

    #[arg(short, long)]
    origin: String,
}

type Cache = Arc<DashMap<String, Vec<u8>>>;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let host = format!(
        "127.0.0.1:{}",
        args.port
    );
    let listener = TcpListener::bind(host).unwrap();
    println!("Server is running at port {:?}", args.port);

    let cache: Cache = Arc::new(DashMap::new());
    let client = Client::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let client = client.clone();
        let cache = cache.clone();
        let origin = args.origin.clone();

        task::spawn(async move {
            handle_connection(stream, client, cache, origin).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, client: Client, cache: Cache, origin: String) {
    let path = get_path(&stream);
    println!("Path: {}", path);

    if !path.is_empty() {
        if let Some(cache_response) = cache.get(&path) {
            println!("Serving from cache...");
            send_response(stream, cache_response.to_vec(), true);
            return;
        }
    }

    if let Ok(response) = forward(&client, &origin, &path).await {
        cache.insert(path.clone(), response.clone());
        send_response(stream, response, false);
    } else {
        let error_message = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
        stream.write_all(error_message.as_bytes()).unwrap();
    }
}

fn get_path(stream: &TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .filter_map(Result::ok)
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(request_line) = http_request.get(0) {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 2 {
            let url = parts[1];
            if let Some((path, query)) = url.split_once("?") {
                for param in query.split('&') {
                    if let Some((key, value)) = param.split_once("=") {
                        println!("Key: {}, Value: {}", key, value);
                    }
                }
                return path.to_string();
            }
            return url.to_string();
        }
    }

    "".to_string()
}

async fn forward(client: &Client, origin: &str, path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = format!("{}{}", origin, path);
    println!("Url: {}", url);

    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Request failed: {:?}", err);
            return Err(Box::new(err));
        }
    };
    println!("Response Status: {}", response.status());

    let body = response.bytes().await?;

    match std::str::from_utf8(&body) {
        Ok(text) => println!("Response Body: {}", text),
        Err(_) => println!("Response Body (Non-UTF-8, in hex): {:?}", body),
    }

    Ok(body.to_vec())
}

fn send_response(mut stream: TcpStream, body: Vec<u8>, cached: bool) {
    let cache_status = if cached { "HIT" } else { "MISS" };
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/octet-stream\r\nX-Cache: {}\r\n\r\n",
        body.len(),
        cache_status
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send headers: {}", e);
        return;
    }
    if let Err(e) = stream.write_all(&body) {
        eprintln!("Failed to send body: {}", e);
    }
}
