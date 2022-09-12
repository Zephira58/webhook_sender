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
    update_check: i32,
    update_available: bool,
    update_notification: bool,
    embed: bool,
    embed_title: String,
    embed_footer: String,
    embed_footer_icon: String,
    embed_image: String,
    embed_thumbnail: String,
    embed_field_title: String,
    embed_field_value: String,
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
            update_check: 0,
            update_available: false,
            update_notification: false,
            embed: false,
            embed_title: "".to_string(),
            embed_footer: "".to_string(),
            embed_footer_icon: "".to_string(),
            embed_image: "".to_string(),
            embed_thumbnail: "".to_string(),
            embed_field_title: "".to_string(),
            embed_field_value: "".to_string(),
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
                ui.label("*Required");
                ui.horizontal(|ui| {
                    ui.label("*Enter Webhook URL:");
                    ui.text_edit_singleline(&mut self.webhook);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Username: ");
                    ui.add_space(25.0);
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Enter Avatar URL: ");
                    ui.add_space(20.0);
                    ui.text_edit_singleline(&mut self.avatar_url);
                });
                ui.horizontal(|ui| {
                    ui.label("*Enter a message:");
                    ui.add_space(21.0);
                    ui.text_edit_multiline(&mut self.message);
                });

                if self.embed {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Title: ");
                        ui.add_space(58.0);
                        ui.text_edit_singleline(&mut self.embed_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Footer: ");
                        ui.add_space(46.0);
                        ui.text_edit_singleline(&mut self.embed_footer);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Footer Icon: ");
                        ui.add_space(20.0);
                        ui.text_edit_singleline(&mut self.embed_footer_icon);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Image URL: ");
                        ui.add_space(24.0);
                        ui.text_edit_singleline(&mut self.embed_image);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Enter Embed Thumbnail URL: ");
                        ui.text_edit_singleline(&mut self.embed_thumbnail);
                    });
                    ui.horizontal(|ui| {
                        ui.label("*Enter Embed Field Title: ");
                        ui.add_space(23.0);
                        ui.text_edit_singleline(&mut self.embed_field_title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("*Enter Embed Field Value: ");
                        ui.add_space(17.0);
                        ui.text_edit_singleline(&mut self.embed_field_value);
                    });
                }

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
                        println!("\nERROR: Webhook URL not found!");
                        cb(self.toasts.error("Please enter a wehbook url"));
                    }
                    else if send_button.clicked() && self.message.is_empty() {
                        println!("\nERROR: Message not found!");
                        cb(self.toasts.error("Please enter a message"));
                    }
                    else if send_button.clicked() && self.embed == false {
                        send_message(&self.message, &self.webhook, &self.username, &self.avatar_url, );
                        cb(self.toasts.success("Message Sent!"));
                        self.message = "".to_string();
                    }
                    else if send_button.clicked() && self.embed_field_title.is_empty() || send_button.clicked() && self.embed_field_value.is_empty() {
                        cb(self.toasts.error("Embed field title and value are required!"));
                        println!("ERROR: Embed field title and value are required!");
                    }
                    else if send_button.clicked() && self.embed == true {
                        send_embed(&self.message, &self.webhook, &self.username, &self.avatar_url, &self.embed_title, &self.embed_footer, &self.embed_footer_icon, &self.embed_image, &self.embed_thumbnail, &self.embed_field_title, &self.embed_field_value);
                        cb(self.toasts.success("Message Sent!"));
                        self.message = "".to_string();
                    }

                    let embed = ui.checkbox(&mut self.embed, "Embed?");
                    if embed.clicked() {
                        &self.embed != &self.embed;
                        println!("\nEmbed: {}", &self.embed);
                        cb(self.toasts.success("Embed toggled!"));
                    }
                });

                //Various UI elements and checks for application updates
                if self.update_check == 0 {
                    cb(self.toasts.info("Checking for updates... Please wait"));
                    println!("\nChecking for updates...");
                }
                if self.update_check == 100 {
                    self.update_check = 110;
                    let x = update();
                    if x.is_err() {
                        self.update_available = true
                    }
                    else {
                        cb(self.toasts.info("No update available"));
                        println!("\nNo update available");
                    }
                }
                else if self.update_check < 105 && !self.update_available {
                    self.update_check += 1;
                }

                if self.update_available {
                    ui.separator();
                    if !self.update_notification {
                        cb(self.toasts.info("Update available!"));
                        println!("\nUpdate available!");
                        self.update_notification = true;
                    }
                    ui.label("There is an update available!");
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
