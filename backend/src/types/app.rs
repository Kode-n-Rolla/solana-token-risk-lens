use crate::birdeye::client::BirdeyeClient;

#[derive(Clone)]
pub struct AppState {
    pub birdeye_client: BirdeyeClient,
}