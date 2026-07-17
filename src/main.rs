use std::sync::Arc;
use russh::{server, Channel, ChannelId};
use russh::server::{ChannelOpenHandle, Handler, Msg, Server, Session};
use russh::keys::Certificate;
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

    async fn channel_open_session(&mut self, _channel: Channel<Msg>, reply: ChannelOpenHandle, _session: &mut Session) -> Result<(), Self::Error> {
        reply.accept().await;
        Ok(())
    }

    async fn shell_request(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
        const MOTD : &str = include_str!("../motd.txt");
        let motd = MOTD.replace('\n', "\r\n");
        session.data(channel, motd.into_bytes())?;
        session.eof(channel)?;
        session.exit_status_request(channel, 0)?;
        session.close(channel)?;
        Ok(())
    }
}

impl Server for MyServer {
    type Handler = MyHandler;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
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