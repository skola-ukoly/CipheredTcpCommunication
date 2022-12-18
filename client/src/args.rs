use clap::Parser;

/// Specify address and port of the tcp server you want to connect to
#[derive(Parser, Debug)]
pub struct Args {
    /// address of the remote tcp server
    pub address: String,
    /// port of the tcp server on remote machine
    pub port: u16,
}


