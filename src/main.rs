use serde::Deserialize;
use std::net::TcpListener;
use tungstenite::server::accept;
#[derive(Deserialize)]
struct MusicData {
    title: String,
    artists: Vec<String>,
    album: Option<String>,
    year: Option<u32>,
    playing: bool,
}

fn main() {
    let mut discord = discord_rpc_client::Client::new(419493676054609959).unwrap();
    discord.start();
    println!("Discord started");
    loop {
        let mut any_set = false;
        for t in windowtitle::WindowIter::new().titles() {
            let t = t.split('-').next();
            if t.is_none() {
                continue;
            }
            let t = t.unwrap();
            if let Ok(music_data) = serde_json::from_str::<MusicData>(t) {
                if music_data.playing {
                    let disc_res =
                        discord.set_activity(|act: discord_rpc_client::models::Activity| {
                            act.details(music_data.title)
                                .state(music_data.artists.join(", "))
                                .assets(|ass| {
                                    ass.small_image("playing")
                                        .small_text("Playing")
                                        .large_image("def")
                                        .large_text("Youtube Music")
                                })
                        });
                    if disc_res.is_err() {
                        println!("Failed to set status.");
                    }
                    any_set = disc_res.is_ok();
                }
            }
        }
        if !any_set {
            discord.clear_activity();
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
