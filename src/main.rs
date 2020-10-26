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
    let server = TcpListener::bind("127.0.0.1:8975").unwrap();
    println!("Discord started");
    loop {
        if let Some(stream) = server.incoming().next() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            println!("Websocket connected");
            loop {
                let t = websocket.read_message().unwrap();
                println!("Got message");
                if t.is_close() {
                    println!("Websocket closed");
                    break;
                } else if !t.is_text() {
                    continue;
                }
                if let Ok(music_data) = serde_json::from_str::<MusicData>(t.to_text().unwrap()) {
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
                    } else {
                        discord.clear_activity();
                    }
                }
            }
        }
    }
}
