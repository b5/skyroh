use std::env;

use anyhow::Context;
use clap::{Parser, Subcommand};
use dotenv::dotenv;

use groundwork::dumbpipe::{CommonArgs, ListenTcpArgs};
use groundwork::peering::Peering;
use groundwork::puppeteer;

/// Create a dumb pipe between two machines, using an iroh magicsocket.
///
/// One side listens, the other side connects. Both sides are identified by a
/// 32 byte node id.
///
/// Connecting to a node id is independent of its IP address. Dumbpipe will try
/// to establish a direct connection even through NATs and firewalls. If that
/// fails, it will fall back to using a relay server.
///
/// For all subcommands, you can specify a secret key using the IROH_SECRET
/// environment variable. If you don't, a random one will be generated.
///
/// You can also specify a port for the magicsocket. If you don't, a random one
/// will be chosen.
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Listen on a magicsocket and forward incoming connections to the specified
    /// host and port. Every incoming bidi stream is forwarded to a new connection.
    ///
    /// Will print a node ticket on stderr that can be used to connect.
    ///
    /// As far as the magic socket is concerned, this is listening. But it is
    /// connecting to a TCP socket for which you have to specify the host and port.
    ListenTcp(groundwork::dumbpipe::ListenTcpArgs),

    /// Connect to a magicsocket, open a bidi stream, and forward stdin/stdout
    /// to it.
    ///
    /// A node ticket is required to connect.
    ///
    /// As far as the magic socket is concerned, this is connecting. But it is
    /// listening on a TCP socket for which you have to specify the interface and port.
    ConnectTcp(groundwork::dumbpipe::ConnectTcpArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let did = env::var("SKYROH_DID").context("missing SKYROH_DID env var")?;
    let password =
        env::var("SKYROH_APP_PASSWORD").context("missing SKYROH_APP_PASSWORD env var")?;

    // let args = Args::parse();
    // let res = match args.command {
    //     Commands::ListenTcp(args) => groundwork::dumbpipe::listen_tcp(args).await,
    //     Commands::ConnectTcp(args) => groundwork::dumbpipe::connect_tcp(args).await,
    // };

    let repo_path = "./";
    let state = Peering::open_or_create(did.as_str(), repo_path).await?;
    println!("{:?}", state);

    let mut pptr = puppeteer::Puppeteer::new("http://localhost:2583").await?;
    pptr.login_bsky(&did, &password).await?;
    pptr.create_local_account().await?;
    let session = pptr.bsky_session().expect("no bsky session");
    pptr.import_bsky_to_local(session.did.clone()).await?;
    // pptr.login_local(&did, &password).await?;
    // let commit = pptr.get_latest_commit().await?;
    // println!("oh snap a commit: {:?}", commit);
    // pptr.make_dummy_acct_and_post().await?;

    // import specified repo
    // let did = Did::new(did).expect("invalid did");
    // pptr.import_repo(did.clone()).await?;

    let args = ListenTcpArgs {
        host: "localhost:8000".to_string(),
        common: CommonArgs::default(),
    };

    let res = groundwork::dumbpipe::listen_tcp(args).await;
    match res {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1)
        }
    }
}
