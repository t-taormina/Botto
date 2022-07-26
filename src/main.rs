/*
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    Client,
};

const HELP_MESSAGE: &str = "
OI OI!

How are we?

You requested some help so lets see what I can do.

❓ Need technical help?
➡️ Post in the <#1001525201533673493> channel and Tyler will get back to you.

❓ Something wrong?
➡️ You can flag an admin with @admin.

I hope that resolves your issue!

— Botto YNWA
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("Received");
        if msg.content == HELP_COMMAND {
            println!("matched");
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    

    let mut client = Client::builder(&token, GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
