use dedup_iter::DedupAdapter;
use lazy_static::lazy_static;
use rand::{prelude::ThreadRng, thread_rng, Rng};
use regex::{Regex, RegexSet};
use std::cell::RefCell;
use std::ops;
use unic::ucd::category::GeneralCategory;
// extern crate azure_sdk_for_rust;
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::channel::ReactionType;
use serenity::prelude::{Context, EventHandler};

use ureq::json;

#[group]
#[commands(ping)]
struct General;

struct Handler;
impl EventHandler for Handler {}

lazy_static! {
    static ref LEMON_REGEX: Regex = Regex::new(include_str!("../regex_lemon.txt")).unwrap();
    static ref MABBS_REGEX: Regex = Regex::new(include_str!("../regex_mabbs.txt")).unwrap();
    static ref ASHER_REGEX: Regex = Regex::new(include_str!("../regex_asher.txt")).unwrap();
    static ref IMAGE_URL_REGEX: Regex = Regex::new(r"(?i)\.(jpe?g|png|tiff|bmp)[^\./]*$").unwrap();
    static ref VENT_REGEX: Regex =
        Regex::new(r"(?i)\b(vent(ing)?|serious|emotional support)\b").unwrap();
    static ref SETTINGS: config::Config = {
        let mut s = config::Config::default();
        s.merge(config::File::with_name("lemonbot_settings"))
            .unwrap()
            .merge(config::Environment::with_prefix("LEMON"))
            .unwrap();
        s
    };
    static ref DISCORD_TOKEN: String = SETTINGS.get_str("discord_token").ok().unwrap();
    static ref AZURE_TOKEN: String = SETTINGS.get_str("azure_token").unwrap();
    static ref AZURE_ENDPOINT: String = SETTINGS.get_str("azure_endpoint").unwrap();
}

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(thread_rng());
}

struct RegResult {
    lemon: bool,
    mabbs: bool,
    asher: bool,
}

impl ops::Add for RegResult {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            lemon: self.lemon || other.lemon,
            mabbs: self.mabbs || other.mabbs,
            asher: self.asher || other.asher,
        }
    }
}

fn get_text_flags(s: &str) -> RegResult {
    let ss = s
        .chars()
        .filter(|c| !GeneralCategory::of(*c).is_mark())
        .dedup()
        .collect::<String>();
    let sss = ss
        .chars()
        .filter(|c| !GeneralCategory::of(*c).is_separator())
        .dedup()
        .collect::<String>();

    let ssss = if sss.len() > 10 { &ss } else { &sss };
    // let regset = RegexSet::new(&[&LEMON_REGEX, &MABBS_REGEX, &ASHER_REGEX]);

    RegResult {
        lemon: LEMON_REGEX.is_match(ssss),
        mabbs: MABBS_REGEX.is_match(ssss),
        asher: ASHER_REGEX.is_match(ssss),
    }
}
fn get_img_flags(msg: &Message, _ctx: &Context) -> RegResult {
    let aurls = msg
        .attachments
        .iter()
        .map(|att| &att.url)
        .filter(|url| IMAGE_URL_REGEX.is_match(url));
    let eurls = msg.embeds.iter().filter_map(|emb| {
        if emb.image.is_some() && IMAGE_URL_REGEX.is_match(&emb.image.as_ref().unwrap().url) {
            Some(&emb.image.as_ref().unwrap().url)
        } else if emb.thumbnail.is_some()
            && IMAGE_URL_REGEX.is_match(&emb.thumbnail.as_ref().unwrap().url)
        {
            Some(&emb.thumbnail.as_ref().unwrap().url)
        } else {
            None
        }
    });
    let urls = aurls.chain(eurls).take(5);

    for url in urls {
        //println!("image url? {}", url);
        let resp = ureq::post(&format!(
            "{}/vision/v3.0/analyze?visualFeatures=Tags",
            AZURE_ENDPOINT.as_str()
        ))
        .set("Ocp-Apim-Subscription-Key", AZURE_TOKEN.as_str())
        .send_json(json!({ "url": url }));
        if resp.ok() {
            let json = resp.into_json().unwrap();
            let tags = json["tags"].as_array().unwrap();
            for t in tags.iter() {
                if t["name"].as_str().unwrap().contains("lemon") {
                    return RegResult {
                        lemon: true,
                        mabbs: false,
                        asher: false,
                    };
                }
            }
        } else {
            println!("error {}: {}", resp.status(), resp.into_string().unwrap());
        }
    }
    RegResult {
        lemon: false,
        mabbs: false,
        asher: false,
    }
}


fn react_msg<R: Into<ReactionType>>(ctx: &Context, msg: &Message, r: R, dorand: bool) {
    if (!dorand || RNG.with(|r| r.borrow_mut().gen_range(0, 5) == 0)) && msg.react(ctx, r).is_err()
    {
        println!("Error reacting to message {}", msg.id);
    };
}

fn handle_message(ctx: &mut Context, msg: &Message) {
    if msg.is_own(&ctx) {
        return;
    }
    if let Some(channel) = msg.channel(&ctx) {
        let guild = channel.guild();
        if guild.is_some() && VENT_REGEX.is_match(&guild.unwrap().read().name) {
            return;
        }
    }

    let mut res = get_text_flags(&msg.content_safe(&ctx));

    for emb in &msg.embeds {
        if emb.description.is_some() {
            res = res + get_text_flags(emb.description.as_ref().unwrap());
        }
        if emb.author.is_some() {
            res = res + get_text_flags(&emb.author.as_ref().unwrap().name);
        }
    }

    if !res.lemon {
        res = res + get_img_flags(msg, ctx);
    }

    if res.lemon {
        react_msg(ctx, msg, '🍋', false);
        return;
    }
    if res.mabbs {
        react_msg(ctx, msg, '🍋', true);
        return;
    }
    // if res.asher {
        // react_msg(ctx, msg, '🍋', true);
        // return;
    // }
}

fn main() {
    assert!(get_text_flags("lemon").lemon);
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("lemonbot_settings"))
        .unwrap()
        .merge(config::Environment::with_prefix("LEMON"))
        .unwrap();

    let mut client = Client::new(DISCORD_TOKEN.as_str(), Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .normal_message(handle_message)
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
