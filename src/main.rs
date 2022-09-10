#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;
use app::*;

use eframe::egui::{Vec2, Visuals};

pub const APP_NAME: &str = "Webhook Sender";

fn main() {
    let mut options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,                                   //app name
        options,                                    //just leave this at options
        Box::new(|_cc| Box::new(MyApp::default())), //leave this as default
    );
}
