pub mod cli;

use clap::Parser;
use cli::*;
use std::{
    io::{self, Write},
    net::TcpStream,
    sync::{
        Arc,
        mpsc::{Sender, channel},
    },
    time::SystemTime,
    thread,
};

static FLUSH_INTERVAL_MS: u128 = 1000;

fn scan(transmitter: Sender<u16>, start_port: u16, args: &Cli) {
    let mut port: u16 = start_port + 1;

    let mut last_flush = SystemTime::now();

    loop {
        match TcpStream::connect((args.ip_addr, port)) {
            Ok(_) => {
                print!(".");
                transmitter.send(port).unwrap();

                let now = SystemTime::now();

                if now
                    .duration_since(last_flush)
                    .expect("Time went backwards")
                    .as_millis() >= FLUSH_INTERVAL_MS {
                    io::stdout().flush().unwrap();
                    last_flush = now;
                }
            }
            Err(_) => {}
        };

        if (args.max_port - port) <= args.threads {
            break;
        }

        port += args.threads;
    }
}

fn main() {
    let args = Arc::new(Cli::parse());

    let (transmitter, receiver) = channel();

    for i in 0..args.threads {
        let transmitter = transmitter.clone();
        let args = Arc::clone(&args);

        thread::spawn(move || {
            scan(transmitter, i, &*args);
        });
    }

    drop(transmitter);

    let mut out = vec![];

    for port in receiver {
        out.push(port);
    }

    println!("");
    out.sort();

    for port in out {
        println!("{} is open", port);
    }
}
