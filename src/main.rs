use self::core::recommendations::RecommendedSong;
use std::{
    collections::{LinkedList, VecDeque},
    fmt::format,
};
use gui::start_gui;

use std::sync::mpsc;

mod core;
mod gui;
extern crate eframe;
extern crate egui;
extern crate rand;

#[tokio::main]
async fn main() {
    start_gui();
}
