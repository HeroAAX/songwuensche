use std::sync::mpsc::Receiver;

use egui::{Button, ScrollArea};

use crate::core::{
    recommendations::RecommendedSong, songrecommendationsconfig::SongRecommendationsConfig,
};

impl eframe::App for Songwuensche {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(mut s) = self.updated_songs.try_recv() {
            self.recommended_songs.append(&mut s);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut index = 0;
                for recommendation in self.recommended_songs.clone() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:03}  ", recommendation.id));
                        if ui.add(create_delete_button()).clicked() {
                            self.recommended_songs.remove(index);
                        }

                        ui.label(format!("{}:   ", recommendation.user));

                        ui.hyperlink_to(recommendation.title, recommendation.url);
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

pub struct Songwuensche {
    recommended_songs: Vec<RecommendedSong>,
    updated_songs: Receiver<Vec<RecommendedSong>>,
    _songrecommendation_config: SongRecommendationsConfig,
}

impl Songwuensche {
    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    pub fn new(
        updated_songs: Receiver<Vec<RecommendedSong>>,
        songrecommendation_config: SongRecommendationsConfig,
    ) -> Self {
        let recommended_songs = vec![];
        Self {
            updated_songs,
            recommended_songs,
            _songrecommendation_config: songrecommendation_config,
        }
    }
}
