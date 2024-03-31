#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, Vec2};
use layout::ROW_MAX_LENGTH;
use uuid::Uuid;
use std::collections::HashMap;

mod map_element;
use map_element::*;

mod json_helper;
mod layout;

mod row_element;
use row_element::*;

mod autotracker;

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
    entrances: HashMap<Uuid, MapElement>,
    placed_icons: HashMap<Uuid, MapElement>,
    row_icons: [[RowElement; ROW_MAX_LENGTH]; 4],
    drag_element: Option<MapElement>
}

impl MyApp {
    fn new() -> Self {
        let data = include_str!("../assets/map/750.json");
        return MyApp {
            entrances: json_helper::load(data),
            placed_icons: HashMap::new(),
            row_icons: [
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW1[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW2[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW3[i])),
                core::array::from_fn(|i| map_element_kind_to_row_element(layout::ROW4[i]))
            ],
            drag_element: None
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
            let map_size = egui::Vec2::new(750.0, 750.0);
            let rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), map_size);
            ui.put(rect, egui::Image::new(egui::include_image!("../assets/lightworld750.png")));

            let rect = egui::Rect::from_min_size(egui::Pos2::new(750.0, 0.0), map_size);
            ui.put(rect, egui::Image::new(egui::include_image!("../assets/darkworld750.png")));

            let x_start = 26.0;
            let y_start = 776.0;
            let offset = 49.0;
            let icon_size = egui::Vec2::new(44.0, 44.0);

            let mut x = x_start;
            let mut y = y_start;
            for row in &mut self.row_icons {
                for item in row {
                    let item_pos = egui::Pos2::new(x, y);
                    x += offset;

                    if item.kind == MapElementKind::BLANK { continue; }

                    let rect = egui::Rect::from_center_size(item_pos, icon_size);
                    let widget = egui::Image::new(kind_to_source(item.kind))
                        .sense(egui::Sense::click_and_drag());

                    let response = ui.put(rect, widget);
                    if response.secondary_clicked() {
                        item.state = RowElementState::CROSSED;
                    }
                    if response.middle_clicked() {
                        item.state = RowElementState::CIRCLED;
                    }

                    if response.hovered() {
                        let category = kind_to_category(&item.kind);
                        for (_k,v) in self.placed_icons.iter().filter(|&(_k,v)| category == kind_to_category(&v.kind)) {
                            ui.painter().line_segment([item_pos, v.pos], egui::Stroke::new(5.0, Color32::RED));
                        }
                    }

                    if item.state == RowElementState::CROSSED {
                        ui.put(rect, egui::Image::new(egui::include_image!("../assets/icons/cross.png")));
                    } else if item.state == RowElementState::CIRCLED {
                        ui.put(rect, egui::Image::new(egui::include_image!("../assets/icons/todo.png")));
                    }

                    if response.drag_started() {
                        let mut new_drag = map_element_kind_to_map_element(item.kind);
                        new_drag.size = Some(Vec2::new(25.0, 25.0));
                        
                        self.drag_element = Some(new_drag);
                    }
                    else if response.drag_released() {
                        let new_item = MapElement {
                            pos: ui.input(|i| i.pointer.latest_pos().expect("Expected a pointer while dragging")),
                            size: self.drag_element.expect("Drag Element should not be null").size,
                            kind: self.drag_element.expect("Drag Element should not be null").kind,
                            id: Uuid::new_v4(),
                            checked: false
                        };

                        self.placed_icons.insert(new_item.id, new_item);
                        self.drag_element = None;
                    }
                }

                x = x_start;
                y += offset;
            }

            let mut drag_started_item: Option<MapElement> = None;
            let mut drag_released_item: Option<MapElement> = None;
            let mut disabled_item: Option<MapElement> = None;
            for (_id, item) in &mut self.entrances {
                let icon_size = egui::Vec2::new(15.0, 15.0);
                let rect = egui::Rect::from_center_size(item.pos, icon_size);
                
                let widget = egui::Image::new(kind_to_source(item.kind))
                    .sense(egui::Sense::click());
                    // .tint(Color32::from_white_alpha(if item.1.checked { 25 } else { 255 } ));

                let response = ui.put(rect, widget);
                if response.secondary_clicked() {
                    disabled_item = Some(*item);
                }
            }

            for (_id, item) in &mut self.placed_icons {
                let icon_size = item.size.expect("Placed icons should have a size");
                let rect = egui::Rect::from_center_size(item.pos, icon_size);
                
                let widget = egui::Image::new(kind_to_source(item.kind))
                    .sense(egui::Sense::click_and_drag());

                let response = ui.put(rect, widget);
                if response.secondary_clicked() {
                    disabled_item = Some(*item);
                }

                if response.drag_started() && kind_is_dragable(item.kind) {
                    self.drag_element = Some(*item);
                    drag_started_item = self.drag_element;
                }
                else if response.drag_released() && self.drag_element.is_some() {
                    drag_released_item = Some(MapElement {
                        pos: ui.input(|i| i.pointer.latest_pos().expect("Expected a pointer while dragging")),
                        size: self.drag_element.expect("Drag Element should not be null").size,
                        kind: self.drag_element.expect("Drag Element should not be null").kind,
                        id: Uuid::new_v4(),
                        checked: false
                    });
                    self.drag_element = None;
                }
            }

            match drag_started_item {
                Some(drag_item) => self.placed_icons.remove(&drag_item.id),
                None => None
            };
            match drag_released_item {
                Some(drag_item) => self.placed_icons.insert(drag_item.id, drag_item),
                None => None
            };
            match disabled_item {
                Some(disable_item) => {
                    self.entrances.remove(&disable_item.id);
                    self.placed_icons.remove(&disable_item.id);
                }
                None => ()
            };

            match self.drag_element {
                Some(drag_element) => {
                    let mouse_position = ui.input(|i| i.pointer.latest_pos().expect("Expected a pointer while dragging"));
                    let rect = egui::Rect::from_center_size(mouse_position, drag_element.size.expect("Drag Element should have a size"));
                    ui.put(rect, egui::Image::new(kind_to_source(drag_element.kind)));
                }
                None => ()
            }
        });
    }
}