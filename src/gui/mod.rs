use std::{collections::LinkedList, sync::mpsc::Receiver};

use egui::{Button, ScrollArea};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, transport::tcp::TCPTransport,
    transport::tcp::TLS, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use crate::core::{
    recommendations::{RecommendedSong, SongRecommendationSolver},
    songrecommendationsconfig::{self, load_config, SongRecommendationsConfig},
};

impl eframe::App for Songwuensche {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(ServerMessage::Privmsg(message)) = self.incoming_messages.try_recv() {
            self.recommended_songs
                .append(&mut self.solver.extract_recommendation(message));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut index = 0;
                for recommendation in self.recommended_songs.clone() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:03}  ", recommendation.id));
                        ui.label(format!("{}:   ", recommendation.user));

                        ui.hyperlink_to(recommendation.title, recommendation.url);

                        if ui.add(create_delete_button()).clicked() {
                            self.recommended_songs.remove(index);
                        }
                    });
                    index += 1;
                }
            });

            ui.add_visible_ui(true, |_| {
                let scroll = ScrollArea::new([true, false]);
                scroll
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                    .enable_scrolling(true)
                    .max_width(320.0)
                    .max_height(f32::INFINITY)
            });
        });

        ctx.request_repaint();
    }

    fn on_close_event(&mut self) -> bool {
        true
    }
}

fn create_delete_button() -> Button {
    let button = Button::new("X");
    button
        .frame(false)
        .fill(egui::Color32::from_rgb(182, 86, 81))
}

pub fn start_gui() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Songwünsche",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::<Songwuensche>::default()
        }),
    )
    .unwrap();
}

struct Songwuensche {
    recommended_songs: Vec<RecommendedSong>,
    incoming_messages: UnboundedReceiver<ServerMessage>,
    _client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
    solver: SongRecommendationSolver,
    _songrecommendation_config: SongRecommendationsConfig,
}

impl Default for Songwuensche {
    fn default() -> Self {
        let config = ClientConfig::default();
        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let songrecommendation_config = load_config();

        //println!("joining {}...", &songrecommendation_config.channel_id);
        client
            .join(songrecommendation_config.channel_id.clone())
            .unwrap();

        let recommended_songs = vec![];
        Self {
            recommended_songs,
            incoming_messages,
            _client: client,
            solver: SongRecommendationSolver::new(songrecommendation_config.filter_urls.clone()),
            _songrecommendation_config: songrecommendation_config,
        }
    }
}
