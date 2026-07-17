use std::sync::Arc;
use rand::random;
use russh::server;
use russh::server::{Auth, Config, Handler, RunningServer, Server};
use russh::server::Auth::Accept;
use russh::keys::Certificate;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::net::unix::SocketAddr;
use russh::keys::PrivateKey;
use ssh_key::Algorithm;

struct MyServer;
struct MyHandler;

impl Handler for MyHandler {
    type Error = russh::Error;
    async fn auth_publickey(
        &mut self,
        _: &str,
        _key: &ssh_key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Accept)
    }

    async fn auth_openssh_certificate(
        &mut self,
        _user: &str,
        _certificate: &Certificate,
    ) -> Result<server::Auth, Self::Error> {
        Ok(server::Auth::Accept)
    }
}

impl Server for MyServer {
    type Handler = MyHandler;

    fn new_client(&mut self, peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        MyHandler
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = server::Config::default();
    let pubkey = PrivateKey::random(
        &mut rand::rng(),
        Algorithm::Ed25519
    )?;
    config.keys.push(pubkey);
    let config = Arc::new(config);

    let mut server = MyServer;

    server.run_on_address(config, "0.0.0.0:2222").await?;
    Ok(())
}