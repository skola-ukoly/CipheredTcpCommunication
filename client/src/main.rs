#![allow(unused)]

mod error;
mod args;
mod communication;
mod ui;
mod middleware;

use middleware::Middleware;
use communication::*;

use clap::Parser;

fn main() {
    let args = args::Args::parse();

    let middleware = Middleware::new();
    let stream = Stream::connect_to_server(args.address, args.port).expect("could not establish connection with server");

    let handle = create_reader_thread(&stream, &middleware);
    

    handle.join(); 
}

