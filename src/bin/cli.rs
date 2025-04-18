use peer::{clients::Client, DEFAULT_PORT};

use bytes::Bytes;
use clap::{Parser, Subcommand};
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(
    name = "peer-cli",
    version,
    author,
    about = "Issue Redis commands"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        /// Message to ping
        #[clap(value_parser = bytes_from_str)]
        msg: Option<Bytes>,
    },
    /// Get the value of key.
    Get {
        /// Name of key to get
        key: Bytes,
    },
    Set {
        /// Name of key to set
        key: Bytes,

        /// Value to set.
        // #[clap(value_parser = bytes_from_str)]
        value: Bytes,

        /// Expire the value after specified amount of time
        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
    Peer {
        // Node subcommand
        #[clap(subcommand)]
        command: PeerCommand,
    }
}

#[derive(Subcommand, Debug)]
enum PeerCommand {
    Basic,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> peer::Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Get the remote address to connect to
    let addr = format!("{}:{}", cli.host, cli.port);

    // Establish a connection
    let mut client = Client::connect(&addr).await?;

    // Process the requested command
    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(string) = str::from_utf8(&value) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Get { key } => {
            if let Some(value) = client.get(key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            client.set(key, value).await?;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            client.set_expires(key, value, expires).await?;
            println!("OK");
        }
        Command::Peer {
            command,
        } => {
            match command {
                PeerCommand::Basic => {
                    if let Some(value) = client.peer_basic().await? {
                        if let Ok(string) = str::from_utf8(&value) {
                            println!("\"{}\"", string);
                        } else {
                            println!("{:?}", value);
                        }
                    } else {
                        println!("(nil)");
                    }
                }
            }
        }
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Result<Bytes, Infallible> {
    Ok(Bytes::from(src.to_string()))
}
