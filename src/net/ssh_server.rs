use std::sync::Arc;
use russh::{server, Channel, ChannelId};
use russh_keys::key;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct GameServer {
    pub clients: Arc<Mutex<HashMap<usize, ChannelId>>>,
}

impl server::Server for GameServer {
    type Handler = GameClient;

    fn new_client(&mut self, _peer_addr: Option<std::net::SocketAddr>) -> Self::Handler {
        GameClient {}
    }
}

pub struct GameClient {}

#[async_trait::async_trait]
impl server::Handler for GameClient {
    type Error = anyhow::Error;

    async fn channel_open_session(
        &mut self,
        _channel: Channel<server::Msg>,
        _session: &mut server::Session,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _public_key: &key::PublicKey,
    ) -> Result<server::Auth, Self::Error> {
        // Warning: Accepting all keys for prototype!
        Ok(server::Auth::Accept)
    }
    
    async fn data(
        &mut self, 
        _channel: ChannelId, 
        _data: &[u8], 
        _session: &mut server::Session
    ) -> Result<(), Self::Error> {
        // Handle input data (e.g., keystrokes)
        Ok(())
    }
}

pub async fn run_server(_port: u16) -> anyhow::Result<()> {
    let _config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        ..Default::default()
    };
    let _config = Arc::new(_config);

    // Generate a temporary key for the server (In prod, load from file)
    let _key = russh_keys::key::KeyPair::generate_ed25519().unwrap();
    
    let _sh = GameServer {
        clients: Arc::new(Mutex::new(HashMap::new())),
    };

    // println!("Starting SSH server on port {}", port);
    // russh::server::run(config, &key, sh).await?;
    println!("SSH server mode is currently disabled due to API migration.");
    
    Ok(())
}
