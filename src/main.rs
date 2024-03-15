#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32};
use layout::ROW_MAX_LENGTH;
use uuid::Uuid;
use std::collections::HashMap;

mod map_element;
use map_element::*;

mod json_helper;
mod layout;

mod row_element;
use row_element::*;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 950.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rustrando",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new())
        }),
    )
}

struct MyApp {
    items: HashMap<Uuid, MapElement>,
    row_icons: [[RowElement; ROW_MAX_LENGTH]; 4],
}

impl MyApp {
    fn new() -> Self {
        let data = include_str!("../assets/map/750.json");
        return MyApp {
            items: json_helper::load(data),
            row_icons: [
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW1[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW2[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW3[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW4[i]))
            ]
        };
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let my_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            outer_margin: egui::style::Margin { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 },
            rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::BLACK },
            fill: Color32::from_rgb(20, 20, 20),
            stroke: egui::Stroke::new(0.0, Color32::BLACK),
        };

        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            let vec = egui::Vec2::new(750.0, 750.0);
            let rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), vec);
            ui.put(rect, egui::Image::new(egui::include_image!("../assets/lightworld750.png")));

            let rect = egui::Rect::from_min_size(egui::Pos2::new(750.0, 0.0), vec);
            ui.put(rect, egui::Image::new(egui::include_image!("../assets/darkworld750.png")));

            let x_start = 4.0;
            let y_start = 754.0;
            let offset = 49.0;
            let icon_size = egui::Vec2::new(44.0, 44.0);

            let mut x = x_start;
            let mut y = y_start;
            for row in &mut self.row_icons {
                for item in row {
                    let rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), icon_size);
                    let widget = egui::Image::new(kind_to_source(&item.kind))
                        .sense(egui::Sense::click());

                    let response = ui.put(rect, widget);
                    if response.secondary_clicked() {
                        item.state = RowElementState::CROSSED;
                    }
                    if response.middle_clicked() {
                        item.state = RowElementState::CIRCLED;
                    }

                    if (item.state == RowElementState::CROSSED) {
                        ui.put(rect, egui::Image::new(egui::include_image!("../assets/icons/cross.png")));
                    } else if (item.state == RowElementState::CIRCLED) {
                        ui.put(rect, egui::Image::new(egui::include_image!("../assets/icons/todo.png")));
                    }

                    x += offset;
                }

                x = x_start;
                y += offset;
            }

            for item in &mut self.items {
                let vec = egui::Vec2::new(15.0, 15.0);
                let rect = egui::Rect::from_center_size(item.1.pos, vec);
                
                let widget = egui::Image::new(kind_to_source(&item.1.kind))
                    .sense(egui::Sense::click())
                    .tint(Color32::from_white_alpha(if item.1.checked { 25 } else { 255 } ));

                let response = ui.put(rect, widget);
                if response.secondary_clicked() {
                    item.1.checked = !item.1.checked;
                }
            }
        });
    }
}