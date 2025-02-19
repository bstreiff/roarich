use libxivdat::dat_file::DATFile;
use libxivdat::xiv_gearset::{read_gearset, GearsetFlags, GearsetList};

use crate::data_provider::DataProvider;
use crate::exporters::get_xivgear_json;
use crate::iw_provider::IronworksProvider;

use egui_file::FileDialog;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

//use egui::scroll_area::ScrollBarVisibility;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PersistedData {
    // Example stuff
    last_opened_file: Option<PathBuf>,
}

pub struct RoarichApp {
    persisted_data: PersistedData,

    gearset_config: Option<GearsetList>,

    data_provider: Option<IronworksProvider>,

    open_file_dialog: Option<FileDialog>,

    selected_gearset: i8,

    export_window_open: bool,
    export_window_string: Option<String>,
}

impl Default for PersistedData {
    fn default() -> Self {
        Self {
            last_opened_file: None,
        }
    }
}

const EQUIPMENT_SLOT_NAMES: [&str; 14] = [
    "Primary",
    "Offhand",
    "Head",
    "Body",
    "Hand",
    "Belt",
    "Legs",
    "Feet",
    "Earrings",
    "Necklace",
    "Wrist",
    "Left Ring",
    "Right Ring",
    "Soul Crystal",
];

impl RoarichApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let persisted = match cc.storage {
            Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            None => Default::default(),
        };

        Self {
            persisted_data: persisted,
            gearset_config: None,
            data_provider: Some(IronworksProvider::new()),
            open_file_dialog: None,
            selected_gearset: -1,
            export_window_open: false,
            export_window_string: None,
        }
    }

    pub fn run_load_dialog(&mut self, ctx: &egui::Context) {
        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    let path = file.to_path_buf();

                    let mut dat_file = match DATFile::open(&path) {
                        Ok(dat_file) => dat_file,
                        Err(_x) => {
                            /* we should do something here */
                            return;
                        }
                    };
                    let a_gearset = match read_gearset(&mut dat_file) {
                        Ok(a_gearset) => Some(a_gearset),
                        Err(_x) => None,
                    };

                    self.gearset_config = a_gearset;
                    self.selected_gearset = self.gearset_config.as_ref().unwrap().current;
                    self.persisted_data.last_opened_file = Some(path);
                }
            }
        }
    }
}

impl eframe::App for RoarichApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.persisted_data);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use egui_extras::{Column, TableBuilder};

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.export_window_open {
            egui::Window::new("Export")
                .open(&mut self.export_window_open)
                .resizable([true, true])
                .show(ctx, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Paste the following JSON into ");
                        ui.hyperlink_to(
                            "xivgear's import page",
                            "https://xivgear.app/?page=importsheet",
                        );
                    });
                    ui.separator();
                    ui.label(self.export_window_string.as_ref().unwrap());
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            if self.export_window_open {
                ui.disable();
            }

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            let filter = Box::new({
                                let ext = Some(OsStr::new("DAT"));
                                move |path: &Path| -> bool { path.extension() == ext }
                            });
                            let mut dialog =
                                FileDialog::open_file(self.persisted_data.last_opened_file.clone())
                                    .show_files_filter(filter);
                            dialog.open();
                            self.open_file_dialog = Some(dialog);
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("sidebar")
            .min_width(200.0)
            .max_width(200.0)
            .show(ctx, |ui| {
                if self.export_window_open {
                    ui.disable();
                }

                let available_height = ui.available_height();
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::remainder())
                    .column(Column::auto())
                    .column(Column::auto())
                    .min_scrolled_height(0.0)
                    .max_scroll_height(available_height)
                    .sense(egui::Sense::click());

                table.body(|mut body| {
                    if let Some(gsc) = &self.gearset_config {
                        for gs in gsc.gearsets.iter() {
                            if (gs.flags & GearsetFlags::Exists) == GearsetFlags::Exists {
                                body.row(18.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!("{}", gs.set_number));
                                    });
                                    row.col(|ui| {
                                        ui.label(gs.name.clone());
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("*{}", gs.average_item_level));
                                    });
                                    row.col(|ui| {
                                        ui.label(format!("{}", gs.glamour_plate));
                                    });

                                    if row.response().clicked() {
                                        self.selected_gearset = gs.set_number as i8;
                                    }
                                });
                            }
                        }
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if self.export_window_open {
                ui.disable();
            }

            if let Some(gsc) = &self.gearset_config {
                let index = self.selected_gearset as usize;
                let gs = &gsc.gearsets[index];

                ui.heading(gs.name.clone());

                if ui.button("Export to xivgear").clicked() {
                    self.export_window_open = true;
                    self.export_window_string =
                        Some(get_xivgear_json(&gs, self.data_provider.as_ref().unwrap()));
                }

                let table = TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .column(Column::remainder());

                table.body(|mut body| {
                    for (i, eq) in gs.equipment.iter().enumerate() {
                        // Skip belt slot if it's empty.
                        if i == 5 && eq.item_id == 0 {
                            continue;
                        }

                        let item = self
                            .data_provider
                            .as_ref()
                            .unwrap()
                            .get_item(eq.item_id)
                            .unwrap();

                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", EQUIPMENT_SLOT_NAMES[i]));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", item.name));
                            });
                        });
                    }
                });
            }

            self.run_load_dialog(ctx);

            //ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            //   /egui::warn_if_debug_build(ui);
            //});
        });
    }
}
