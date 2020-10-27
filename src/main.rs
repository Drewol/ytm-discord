use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::net::TcpListener;
use tungstenite::server::accept;
#[derive(Deserialize, Debug, Clone)]
struct MusicData {
    title: String,
    artists: Vec<String>,
    album: Option<String>,
    year: Option<u32>,
    playing: bool,
}

fn main() {
    let args = clap::App::new("YTM Discord Presence")
        .version("0.1")
        .author("Emil \"Drewol\" Draws")
        .about(
            "Websocket server designed to listen to a client running on your youtube music page.",
        )
        .arg(
            clap::Arg::new("bind")
                .short('b')
                .long("bind")
                .about("Address to bind the server to")
                .default_value("127.0.0.1:8975"),
        )
        .arg(
            clap::Arg::new("v")
                .short('v')
                .multiple(true)
                .about("Verbosity")
                .takes_value(false),
        )
        .arg(clap::Arg::new("quiet").short('q').about("Silence output"))
        .get_matches();

    let verbosity = if args.occurrences_of("quiet") == 0 {
        match args.occurrences_of("v") {
            0 => log::LevelFilter::Info,
            1 => log::LevelFilter::Debug,
            2 | _ => log::LevelFilter::Trace,
        }
    } else {
        log::LevelFilter::Off
    };

    SimpleLogger::new().with_level(verbosity).init().unwrap();
    let mut discord = discord_rpc_client::Client::new(419493676054609959).unwrap();
    discord.start();
    let server = TcpListener::bind(args.value_of("bind").unwrap()).unwrap();
    log::info!("Discord started");
    for stream in server.incoming() {
        let websocket_res = accept(stream.unwrap());
        if websocket_res.is_err() {
            continue;
        }
        let mut websocket = websocket_res.unwrap();
        log::info!("Websocket connected");
        loop {
            let t = websocket.read_message().unwrap();
            log::debug!("Got message");
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
            let music_data: MusicData = music_data_res.unwrap();
            let md = music_data.clone();
            if music_data.playing {
                let disc_res = discord.set_activity(|act: discord_rpc_client::models::Activity| {
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
                    log::warn!("Failed to set status: {:?}", md);
                }
            } else {
                if discord.clear_activity().is_err() {
                    log::warn!("Failed to clear status.");
                }
            }
        }
    }
}
