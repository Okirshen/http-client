use crate::{Method, Response};
use anyhow::Error;
use reqwest::blocking::Client;
use strum::IntoEnumIterator;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    url: String,
    method: Method,
    #[serde(skip)]
    client: Client,
    #[serde(skip)]
    response: Option<Response>,
    #[serde(skip)]
    errors: Vec<Error>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            url: "http://localhost:8000/".to_owned(),
            method: Method::GET,
            client: Client::new(),
            response: None::<Response>,
            errors: Vec::new(),
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            url,
            method,
            client,
            response,
            errors,
        } = self;

        if !errors.is_empty() {
            egui::TopBottomPanel::top("error_panel").show(ctx, |ui| {
                errors.retain(|error| {
                    ui.horizontal(|ui| {
                        ui.label(error.to_string());
                        return !ui.button("X").clicked();
                    })
                    .inner
                });
            });
        }
        // egui::SidePanel::left("side_panel").show(ctx, |ui| {});

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("")
                    .selected_text(format!("{}", method))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        for i in Method::iter() {
                            ui.selectable_value(method, i.to_owned(), i.to_string());
                        }
                    });
                ui.text_edit_singleline(url);
                if ui.button("Send!").clicked() {
                    match client.request((*method).into(), url.to_owned()).send() {
                        Ok(res) => {
                            *response = Some(Response {
                                url: res.url().to_string(),
                                status_code: res.status().into(),
                                status_message: res
                                    .status()
                                    .canonical_reason()
                                    .unwrap()
                                    .to_string(),
                                body: res.text().ok(),
                            })
                        }
                        Err(error) => errors.push(error.into()),
                    }
                }
            });
            if let Some(response) = response {
                ui.separator();
                ui.monospace(&response.url);
                ui.monospace(format!(
                    "{} {}",
                    response.status_code, response.status_message
                ));
                ui.separator();
                if let Some(body) = &response.body {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(body);
                    });
                }
            }
            egui::warn_if_debug_build(ui);
        });
    }
}
