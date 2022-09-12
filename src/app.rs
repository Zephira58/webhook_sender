use std::time::Duration;

use eframe::egui::{self, Visuals, Window};
use egui_notify::{Anchor, Toast, Toasts};

mod api_handler; //Imports the API handler
use api_handler::*;

use crate::APP_NAME;

pub struct MyApp {
    //Enter global values to be used with your app here
    message: String,
    toasts: Toasts,
    closable: bool,
    duration: f32,
    webhook: String,
    username: String,
    avatar_url: String,
    update_check: bool,
    update_available: bool,
    update_notifcation: bool,
}

impl Default for MyApp {
    //defaults for your global values
    fn default() -> Self {
        Self {
            //enter global default values here
            message: "".to_string(),
            toasts: Toasts::default().with_anchor(Anchor::TopRight),
            closable: true,
            duration: 3.5,
            webhook: "".to_string(),
            username: "Xans Webhook Sender".to_string(),
            avatar_url: "https://cdn.discordapp.com/avatars/292971545956188160/eab559efa07f0f3dd13d21ac5f26c4ce.png?size=1024".to_string(),
            update_check: false,
            update_available: false,
            update_notifcation: false,
        }
    }
}

// The env! macro gets the variable at compile time.
const CURRENT_BUILD: &str = env!("CARGO_PKG_VERSION");
const REPO_URL: &str = env!("CARGO_PKG_REPOSITORY");

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui|{
            Window::new(APP_NAME).show(ctx, |ui| {
                ui.style_mut().visuals = Visuals::dark(); // Makes the buttons dark
                ctx.set_visuals(egui::Visuals::dark()); // Make the ui dark
                egui::warn_if_debug_build(ui);

                let cb = |t: &mut Toast| {
                    //Callback for the toast
                    t.set_closable(self.closable)
                        .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
                };

                ui.horizontal(|ui| {
                    ui.label("Current build:");
                    ui.hyperlink_to(CURRENT_BUILD, REPO_URL);
                });

                ui.add_space(10.0);

                self.message = self.message.replace("\n", "");

                ui.label("Hello and welcome to webhook sender!");
                ui.label(
                "You can randomly generate an insult, affirmation or write your own message in the boxes below!\n");
                ui.label("Notice: This application is not affiliated with Discord in any way.\nThe application will say message sent even if the webhook URL is invalid.");

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Enter Webhook URL ");
                    ui.text_edit_singleline(&mut self.webhook);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Username ");
                    ui.add_space(22.0);
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Avatar URL ");
                    ui.add_space(18.0);
                    ui.text_edit_singleline(&mut self.avatar_url);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter a message ");
                    ui.add_space(23.0);
                    ui.text_edit_multiline(&mut self.message);
                });

                ui.separator();

                ui.label("-Your message-");
                ui.label(&self.message);


                ui.horizontal(|ui| {
                    let generate_button = ui.button("Generate an insult");
                    if generate_button.clicked() {
                        self.message = get_insult();
                        cb(self.toasts.success("Generation Successful!")); //Sends a success toast
                    }

                    let generate_affirmation = ui.button("Generate an affirmation");
                    if generate_affirmation.clicked() {
                        self.message = get_affirmation();
                        cb(self.toasts.success("Generation Successful!"));
                    }

                    let send_button = ui.button("Send message");
                    if send_button.clicked() && self.webhook.is_empty(){
                        println!("ERROR: Webhook URL not found!\n");
                        cb(self.toasts.error("Please enter a wehbook url"));
                    }
                    else if send_button.clicked() && self.message.is_empty() {
                        println!("ERROR: Message not found!\n");
                        cb(self.toasts.error("Please enter a message"));
                    }
                    else if send_button.clicked() {
                        send_message(&self.message, &self.webhook, &self.username, &self.avatar_url);
                        cb(self.toasts.success("Message Sent!"));
                        self.message = "".to_string();
                    }
                });

                //Various UI elements and checks for application updates
                if self.update_check == false {
                    self.update_check = true;
                    let x = update();
                    if x.is_err() {
                        self.update_available = true
                    }
                }
                if self.update_available == true {
                    ui.separator();
                    if self.update_notifcation == false {
                        cb(self.toasts.info("Update avalible!"));
                        self.update_notifcation = true;
                    }
                    ui.label("There is an update avalible!");
                    ui.horizontal(|ui| {
                        ui.label("You can download the update from");
                        ui.hyperlink_to("here", "https://github.com/Xanthus58/webhook_sender/releases/latest");
                    });
                }
                self.toasts.show(ctx); // Requests to render toasts
            });
        });
    }
}
