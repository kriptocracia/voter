use nostr_sdk::prelude::*;
use tokio::sync::mpsc;

use crate::config::AppConfig;
use crate::error::{Result, VoterError};
use crate::nostr::events::{Election, ElectionResults};
use crate::nostr::messages::{EcResponse, VoterMessage};

/// Actions produced by the Nostr client for the app event loop.
#[derive(Debug, Clone)]
pub enum NostrAction {
    ElectionUpdate(Election),
    ElectionResult(ElectionResults),
    EcResponse(EcResponse),
    ConnectionStatus(bool),
    Error(String),
}

/// Wraps the nostr-sdk Client for voter-specific operations.
pub struct NostrVoterClient {
    client: Client,
    ec_pubkey: Option<PublicKey>,
}

impl NostrVoterClient {
    /// Create and connect a new Nostr client using the given keys and config.
    pub async fn connect(keys: &Keys, config: &AppConfig) -> Result<Self> {
        let client = Client::new(keys.clone());

        for relay_url in &config.nostr.relays {
            client
                .add_relay(relay_url)
                .await
                .map_err(|e| VoterError::Nostr(format!("failed to add relay {relay_url}: {e}")))?;
        }

        client.connect().await;

        Ok(Self {
            client,
            ec_pubkey: None,
        })
    }

    /// Set the EC's public key (needed to send Gift Wrap messages).
    pub fn set_ec_pubkey(&mut self, pubkey: PublicKey) {
        self.ec_pubkey = Some(pubkey);
    }

    /// Subscribe to election announcements (Kind 35000), results (Kind 35001),
    /// and Gift Wrap messages addressed to us.
    pub async fn subscribe(&self) -> Result<()> {
        let election_filter = Filter::new().kinds(vec![Kind::Custom(35_000), Kind::Custom(35_001)]);

        let gift_wrap_filter = Filter::new().kind(Kind::GiftWrap).pubkey(
            self.client
                .signer()
                .await
                .map_err(|e| VoterError::Nostr(format!("no signer: {e}")))?
                .get_public_key()
                .await
                .map_err(|e| VoterError::Nostr(format!("no public key: {e}")))?,
        );

        self.client
            .subscribe(election_filter, None)
            .await
            .map_err(|e| VoterError::Nostr(format!("subscribe elections failed: {e}")))?;

        self.client
            .subscribe(gift_wrap_filter, None)
            .await
            .map_err(|e| VoterError::Nostr(format!("subscribe gift wrap failed: {e}")))?;

        Ok(())
    }

    /// Send a voter message to the EC via NIP-59 Gift Wrap.
    pub async fn send_to_ec(&self, msg: &VoterMessage) -> Result<()> {
        let ec_pubkey = self
            .ec_pubkey
            .ok_or_else(|| VoterError::Nostr("EC public key not set".to_string()))?;

        let content = serde_json::to_string(msg)?;
        let my_pubkey = self
            .client
            .signer()
            .await
            .map_err(|e| VoterError::Nostr(format!("no signer: {e}")))?
            .get_public_key()
            .await
            .map_err(|e| VoterError::Nostr(format!("no public key: {e}")))?;
        let rumor = EventBuilder::text_note(content).build(my_pubkey);

        self.client
            .gift_wrap(&ec_pubkey, rumor, Vec::<Tag>::new())
            .await
            .map_err(|e| VoterError::Nostr(format!("gift_wrap send failed: {e}")))?;

        Ok(())
    }

    /// Send a voter message to the EC using a specific (throwaway) signer.
    pub async fn send_to_ec_anonymous(
        &self,
        msg: &VoterMessage,
        throwaway_keys: &Keys,
        config: &AppConfig,
    ) -> Result<()> {
        let ec_pubkey = self
            .ec_pubkey
            .ok_or_else(|| VoterError::Nostr("EC public key not set".to_string()))?;

        // Create a temporary client with the throwaway keys
        let anon_client = Client::new(throwaway_keys.clone());
        for relay_url in &config.nostr.relays {
            anon_client
                .add_relay(relay_url)
                .await
                .map_err(|e| VoterError::Nostr(format!("failed to add relay {relay_url}: {e}")))?;
        }
        anon_client.connect().await;

        let content = serde_json::to_string(msg)?;
        let rumor = EventBuilder::text_note(content).build(throwaway_keys.public_key());

        anon_client
            .gift_wrap(&ec_pubkey, rumor, Vec::<Tag>::new())
            .await
            .map_err(|e| VoterError::Nostr(format!("anonymous gift_wrap failed: {e}")))?;

        anon_client.disconnect().await;

        Ok(())
    }

    /// Start listening for Nostr events and forward them as NostrActions.
    /// This should be spawned as a tokio task.
    pub async fn listen(&self, action_tx: mpsc::UnboundedSender<NostrAction>) -> Result<()> {
        let client = self.client.clone();
        let tx = action_tx;

        client
            .handle_notifications(|notification| {
                let tx = tx.clone();
                async move {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        match event.kind {
                            Kind::Custom(35_000) => {
                                if let Ok(election) =
                                    serde_json::from_str::<Election>(event.content.as_str())
                                {
                                    let _ = tx.send(NostrAction::ElectionUpdate(election));
                                }
                            }
                            Kind::Custom(35_001) => {
                                if let Ok(results) =
                                    serde_json::from_str::<ElectionResults>(event.content.as_str())
                                {
                                    let _ = tx.send(NostrAction::ElectionResult(results));
                                }
                            }
                            Kind::GiftWrap => {
                                // Gift wrap unwrapping is handled by the caller
                                // since it needs the client's signer.
                            }
                            _ => {}
                        }
                    }
                    Ok(false) // keep listening
                }
            })
            .await
            .map_err(|e| VoterError::Nostr(format!("notification handler error: {e}")))?;

        Ok(())
    }

    /// Disconnect from all relays.
    pub async fn disconnect(&self) {
        self.client.disconnect().await;
    }

    /// Get a reference to the underlying nostr-sdk Client.
    pub fn inner(&self) -> &Client {
        &self.client
    }
}
