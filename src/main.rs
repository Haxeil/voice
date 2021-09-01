//! Requires the "client", "standard_framework", and "voice" features be enabled in your
//! Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["client", standard_framework", "voice"]
//! ```
use std::env;

use serenity::prelude::Mentionable;
// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

// Import the `Context` to handle commands.
use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready},
    Result as SerenityResult,
};
use serenity::model::id::ChannelId;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn channel_update(&self, _ctx: Context, _old: Option<serenity::model::channel::Channel>, _new: serenity::model::channel::Channel) {
        if let Some(old) = _old {
            println!("{:?}", old.position().unwrap_or(0));
        }
    }

    async fn voice_state_update(&self, ctx: Context, _: Option<serenity::model::id::GuildId>, old: Option<serenity::model::prelude::VoiceState>, new: serenity::model::prelude::VoiceState) {
        let voice_stage_id: u64 = 882001471061757981;
        let text_stage_id: u64 = 882316467448709180;
        let stage_role_id: u64 = 882316855212146738;


        let voice_big_id: u64 = 818645193829122048;
        let text_big_id: u64 = 882316530107437146;
        let big_role_id: u64 = 882316925173121087;
        
        let member = &mut new.member.unwrap();
        let mention = member.user.mention().to_string();

        //Left 
        



        // Joined
        if let Some(old) = old {
            if new.channel_id.is_some() && old.channel_id.unwrap().0 != voice_big_id {
                if voice_big_id == new.channel_id.unwrap().0 {
                    ChannelId(text_big_id).say(&ctx, format!("member: {} just landed!", mention)).await.expect("cant send left msg");
                    member.add_role(&ctx, big_role_id).await.expect("can't add role");
                }

    
            } else if new.channel_id.is_some() && old.channel_id.unwrap().0 != voice_stage_id {
                if voice_stage_id == new.channel_id.unwrap().0 {
                    ChannelId(text_stage_id).say(&ctx, format!("member: {} just landed!", mention)).await.expect("cant send left msg");
                    member.add_role(&ctx, stage_role_id).await.expect("can't add role");
                }
            }
            // left
            if let Some(channel) = new.channel_id {
                if old.channel_id.unwrap().0 == voice_big_id && channel.0 != voice_big_id {

                    ChannelId(text_big_id).say(&ctx, format!("member: {} just left!", mention)).await.expect("cant send left msg");
                    member.remove_role(&ctx, big_role_id).await.expect("can't add role");
                    
                } else if old.channel_id.unwrap().0 == voice_stage_id && channel.0 != voice_stage_id {
                    ChannelId(text_stage_id).say(&ctx, format!("member: {} just left!", mention)).await.expect("cant send left msg");
                    member.remove_role(&ctx, stage_role_id).await.expect("can't add role");
                    
                }

            } else {

                if old.channel_id.unwrap().0 == voice_big_id {

                    ChannelId(text_big_id).say(&ctx, format!("member: {} just left!", mention)).await.expect("cant send left msg");
                    member.remove_role(&ctx, big_role_id).await.expect("can't add role");
                    
                } else if old.channel_id.unwrap().0 == voice_stage_id {

                    ChannelId(text_stage_id).say(&ctx, format!("member: {} just left!", mention)).await.expect("cant send left msg");
                    member.remove_role(&ctx, stage_role_id).await.expect("can't add role");
                    
                }
            }

        } else {
            if voice_big_id == new.channel_id.unwrap().0 {
                ChannelId(text_big_id).say(&ctx, format!("member: {} just landed!", mention)).await.expect("cant send left msg");
                member.add_role(&ctx, big_role_id).await.expect("can't add role");
            } else if voice_stage_id == new.channel_id.unwrap().0 {
                ChannelId(text_stage_id).say(&ctx, format!("member: {} just landed!", mention)).await.expect("cant send left msg");
                member.add_role(&ctx, stage_role_id).await.expect("can't add role");
            }
        }
        


    }
}

#[group]
#[commands(deafen, join, leave, mute, play, ping, undeafen, unmute)]
struct General;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("TOKEN")
        .expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c
        .prefix("->"))
        .group(&GENERAL_GROUP);
       

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });
    
    tokio::signal::ctrl_c().await.expect("couldn't quit");
    println!("Received Ctrl-C, shutting down.");
}

#[command]
#[only_in(guilds)]
async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
    } else {
        if let Err(e) = handler.deafen(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
    } else {
        if let Err(e) = handler.mute(true).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
    }

    Ok(())
}

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        handler.play_source(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Undeafened").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to undeafen in").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            check_msg(msg.channel_id.say(&ctx.http, format!("Failed: {:?}", e)).await);
        }

        check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to unmute in").await);
    }

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
