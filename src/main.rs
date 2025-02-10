use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
    unlocked: bool,
}

struct FortniteCheat {
    client: Client,
}

impl FortniteCheat {
    fn new() -> Self {
        let client = Client::new();
        FortniteCheat { client }
    }

    async fn unlock_item(&self, item: &CosmeticItem) -> Result<(), Box<dyn Error>> {
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(item)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err("Failed to unlock item".into())
        }
    }

    async fn unlock_all(&self, items: Vec<CosmeticItem>) -> Result<(), Box<dyn Error>> {
        for item in items {
            self.unlock_item(&item).await?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cheat = FortniteCheat::new();
    let items = vec![
        CosmeticItem { name: "Epic Outfit".to_string(), item_type: "outfit".to_string(), unlocked: false },
        CosmeticItem { name: "Legendary Emote".to_string(), item_type: "emote".to_string(), unlocked: false },
    ];
    cheat.unlock_all(items).await?;
    Ok(())
}