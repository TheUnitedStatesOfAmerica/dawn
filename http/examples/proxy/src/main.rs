use dawn_http::client::{config::Proxy, Client};
use dawn_model::id::ChannelId;
use futures::future;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let mut config = Client::builder();
    config
        .proxy(Proxy::all("http://localhost:3000")?)
        .proxy_http(true)
        .skip_ratelimiter(true);
    let client = config.build()?;
    let channel_id = ChannelId(620_980_184_606_048_278);

    future::join_all((1u8..=10).map(|x| {
        client
            .create_message(channel_id)
            .content(format!("Ping #{}", x))
    }))
    .await;

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
