use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use serenity::async_trait;

use serenity::all::{Context, CreateMessage, EventHandler, GatewayIntents};
use serenity::json::json;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: serenity::all::Ready) {
        let ctx = Arc::new(ctx);
        let ctx1 = Arc::clone(&ctx);

        println!("cache ready");
        let endpoints = vec![
            "https://otobusum.metkm.win/bus-locations/KM12?city=istanbul",
            "https://otobusum.metkm.win/route-stops/KM12?direction=G",
            "https://otobusum.metkm.win/routes/KM12",
            "https://otobusum.metkm.win/search?q=km",
            "https://otobusum.metkm.win/timetable/KM12?direction=G"
        ];

        tokio::spawn(async move {
            let priv_channel = ctx1.http.create_private_channel(&json!({
                "recipient_id": "262192783702360074",
            }))
                .await
                .unwrap();

            loop {
                for endpoint in &endpoints {
                    println!("making request to endpoint {endpoint}");

                    let response = reqwest::get(*endpoint)
                        .await;

                    if let Err(why) = response {
                        let message= CreateMessage::new()
                            .content(why.to_string());

                        priv_channel.send_message(&ctx1, message)
                            .await
                            .unwrap();
                    }
                }

                println!("made requests. Sleeping");
                tokio::time::sleep(tokio::time::Duration::from_secs(36000)) // 10 hours
                    .await;
            }
        });
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Discord token is expected");

    let intents = GatewayIntents::empty();

    let mut client = serenity::Client::builder(&token, intents)
        .event_handler(Handler {})
        .await
        .expect("error building client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
