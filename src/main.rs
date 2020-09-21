use regex::Regex;
use lazy_static::lazy_static;
use unic::ucd::category::GeneralCategory;
// extern crate azure_sdk_for_rust;
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use ureq::json;

#[group]
#[commands(ping)]
struct General;

struct Handler;
impl EventHandler for Handler {}

lazy_static! {
    static ref LEMON_REGEX:Regex = Regex::new(include_str!("../lemon_regex.txt")).unwrap();
    static ref IMAGE_URL_REGEX:Regex = Regex::new(r"(?i)\.(jpe?g|png|tiff|bmp)[^\./]*$").unwrap();
    static ref VENT_REGEX:Regex = Regex::new(r"(?i)\b(vent(ing)?|serious|emotional support)\b").unwrap();
    static ref SETTINGS:config::Config = {let mut s = config::Config::default();
                                         s.merge(config::File::with_name("lemonbot_settings")).unwrap()
                                                 .merge(config::Environment::with_prefix("LEMON")).unwrap();
                                         s};
    static ref DISCORD_TOKEN:String = SETTINGS.get_str("discord_token").ok().unwrap();
    static ref AZURE_TOKEN:String = SETTINGS.get_str("azure_token").unwrap();
    static ref AZURE_ENDPOINT:String = SETTINGS.get_str("azure_endpoint").unwrap();
    }

fn is_lemon(s:&str) -> bool {
    let ss = s.chars().filter(|c| !GeneralCategory::of(*c).is_mark()).collect::<String>();
    let sss = ss.chars().filter(|c| !GeneralCategory::of(*c).is_separator()).collect::<String>();
    return LEMON_REGEX.is_match(if sss.len()>20 {&ss} else {&sss});
}

fn react_lemon(ctx:&Context, msg:&Message) {
    if msg.react(ctx, '🍋').is_err() {
        println!("Error reacting to message {}", msg.id);
    };
}

fn handle_message(ctx: &mut Context, msg: &Message) {
    if msg.is_own(&ctx) {return;}
    let channel = msg.channel(&ctx);
    if channel.is_some() {
        let guild = channel.unwrap().guild();
        if guild.is_some() && VENT_REGEX.is_match(&guild.unwrap().read().name) {
            return;
        }
    }

    //println!("message found: {}", msg.content_safe(&ctx));
    if is_lemon(&msg.content_safe(&ctx)) {
        react_lemon(ctx, msg);
        return;
    }
    
    for emb in &msg.embeds {
        if emb.description.is_some() &&
           is_lemon(emb.description.as_ref().unwrap()) &&
           is_lemon(&emb.author.as_ref().unwrap().name) {
            react_lemon(ctx, msg);
            return;
        }
    }

    let aurls = msg.attachments.iter().map(|att| &att.url).filter(|url| IMAGE_URL_REGEX.is_match(url));
    let eurls = msg.embeds.iter().filter_map(|emb|
        if emb.image.is_some() && IMAGE_URL_REGEX.is_match(&emb.image.as_ref().unwrap().url) {Some(&emb.image.as_ref().unwrap().url)}
        else if emb.thumbnail.is_some() && IMAGE_URL_REGEX.is_match(&emb.thumbnail.as_ref().unwrap().url) {Some(&emb.thumbnail.as_ref().unwrap().url)}
        else {None}
    );
    let urls = aurls.chain(eurls).take(5);

    for url in urls {
        //println!("image url? {}", url);
        let resp = ureq::post(&format!("{}/vision/v3.0/analyze?visualFeatures=Tags", AZURE_ENDPOINT.as_str()))
        .set("Ocp-Apim-Subscription-Key", AZURE_TOKEN.as_str())
        .send_json(json!({"url": url}));
        if resp.ok() {
            let json = resp.into_json().unwrap();
            let tags = json["tags"].as_array().unwrap();
            for t in tags.iter() {
                if t["name"].as_str().unwrap().contains("lemon") {
                    react_lemon(ctx, msg);
                    return;
                }
            }
        } else {
            println!("error {}: {}", resp.status(), resp.into_string().unwrap());
        }
    }
}

fn main() {
    assert!(is_lemon("lemon"));
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("lemonbot_settings")).unwrap()
            .merge(config::Environment::with_prefix("LEMON")).unwrap();

    let mut client = Client::new(DISCORD_TOKEN.as_str(), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
                          .normal_message(handle_message)
                          .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
