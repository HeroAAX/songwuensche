
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use gui::Songwuensche;
use twitch_irc::message::PrivmsgMessage;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

use crate::core::{
    recommendations::SongRecommendationSolver, songrecommendationsconfig::load_config,
};
use std::sync::mpsc;
use std::{
    collections::{LinkedList, VecDeque},
    fmt::format,
};

mod core;
mod gui;
extern crate eframe;
extern crate egui;
extern crate rand;

#[tokio::main]
async fn main() {
    let songwuensche_config = load_config();
    let songwuensche_clone = songwuensche_config.clone();

    let (sx, tx) = mpsc::channel();

    let config = ClientConfig::default();
    let sx = sx.clone();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    client.join(songwuensche_clone.channel_id).unwrap();

    let join_handle = tokio::spawn(async move {
        let solver = &mut SongRecommendationSolver::new(songwuensche_clone.filter_urls);
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(message) => {
                    println!("Message: {:?}", message);
                    let mut t = solver.extract_recommendation(message).await;

                    for ele in &mut t {
                        ele.fetch_yt_title().await;
                    }

                    sx.send(t).unwrap();
                }
                _rest => println!("rest"),
            }
        }
    });

    println!("created twitch listener");

    join_handle.is_finished();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 720.0)),
        ..Default::default()
    };

    let eframe = eframe::run_native(
        "Songwünsche",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(Songwuensche::new(tx, songwuensche_config))
        }),
    );
    eframe.unwrap();

    println!("closed window");
    join_handle.await.unwrap();
}
