use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::{rc::Rc, net::TcpListener};
use tungstenite::server::accept;
#[derive(Deserialize, Debug, Clone)]
struct MusicData {
    title: Option<String>,
    artist: Option<String>,
    playing: bool,
}

fn main() {
    let args = clap::App::new("YTM Discord Presence")
        .version("0.1")
        .author("Emil \"Drewol\" Draws")
        .about("Websocket server designed to listen to a client running on your youtube music page.")
        .arg(clap::Arg::new("bind")
                .short('b')
                .long("bind")
                .about("Address to bind the server to")
                .default_value("127.0.0.1:8975"),
        )
        .arg(clap::Arg::new("v").short('v').about("Verbose"))
        .arg(clap::Arg::new("quiet").short('q').about("Silence output"))
        .get_matches();

    let verbosity = if args.is_present("v") {
        log::LevelFilter::Trace
    } else if args.is_present("quiet") {
        log::LevelFilter::Off
    } else {
        log::LevelFilter::Info
    };
    SimpleLogger::new().with_level(verbosity).init().unwrap();

    let mut discord = discord_rpc_client::Client::new(419493676054609959).unwrap();
    discord.start();
    log::info!("Discord started");

    let server = TcpListener::bind(args.value_of("bind").unwrap()).unwrap();
    for stream in server.incoming() {
        let websocket_res = accept(stream.unwrap());
        if websocket_res.is_err() {
            continue;
        }
        let mut websocket = websocket_res.unwrap();
        log::info!("Websocket connected");
        loop {
            let t = websocket.read_message().unwrap();
            log::debug!("Got message: {:?}", t);
            if t.is_close() {
                log::info!("Websocket closed");
                break;
            } else if !t.is_text() {
                continue;
            }
            let music_data_res = serde_json::from_str(t.to_text().unwrap());
            if music_data_res.is_err() {
                log::warn!("Failed to deserialize message: {:?}", t);
            }
            let music_data: Rc<MusicData> = Rc::new(music_data_res.unwrap());
            if music_data.playing && music_data.title.is_some() {
                let disc_res = discord.set_activity(|act: discord_rpc_client::models::Activity| {
                    act.details(music_data.title.as_ref().unwrap())
                        .state(music_data.artist.as_ref().unwrap_or(&String::new()))
                        .assets(|ass| {
                            ass.small_image("playing")
                                .small_text("Playing")
                                .large_image("def")
                                .large_text("YouTube Music")
                        })
                });
                if disc_res.is_err() {
                    log::warn!("Failed to set status: {:?}", music_data);
                }
            } else {
                if discord.clear_activity().is_err() {
                    log::warn!("Failed to clear status.");
                }
            }
        }
    }
}
