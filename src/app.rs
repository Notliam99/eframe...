#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(default)]
pub struct Person {
    name: String,
    age: usize,
    has_phone: bool,
}

impl Into<egui::WidgetText> for Person {
    fn into(self) -> egui::WidgetText {
        egui::WidgetText::RichText(egui::RichText::new(self.name))
    }
}

impl PartialEq for &Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age && self.has_phone == other.has_phone
    }
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: "New Person".to_string(),
            age: 13usize,
            has_phone: false,
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WhoHasPhoneApp {
    people: Vec<Person>,
    add_person_modal_open: bool,
    mut_person: Person,
}

impl Default for WhoHasPhoneApp {
    fn default() -> Self {
        Self {
            people: Vec::new(),
            add_person_modal_open: false,
            mut_person: Person::default(),
        }
    }
}

impl WhoHasPhoneApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for WhoHasPhoneApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if self.add_person_modal_open {
            let add_person_modal = egui::Modal::new(egui::Id::new("Add Person")).show(ctx, |ui| {
                ui.set_width(350f32);

                ui.heading("Add Person");

                ui.horizontal(|ui| {
                    ui.label("Name:".to_string());

                    ui.text_edit_singleline(&mut self.mut_person.name);
                });

                ui.horizontal(|ui| {
                    ui.label("Age:".to_string());
                    egui::ComboBox::from_label("").show_index(
                        ui,
                        &mut self.mut_person.age,
                        100,
                        |i| i.to_string(),
                    )
                });

                ui.checkbox(&mut self.mut_person.has_phone, "Has a Phone?");

                if ui.button("Submit").clicked() {
                    self.people.push(self.mut_person.clone());
                    self.mut_person = Person::default();
                    log::info!("{:?}", &self.people);
                }
            });

            if add_person_modal.should_close() {
                self.add_person_modal_open = false;
                self.mut_person = Person::default();
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                if ui
                    .button("Add")
                    .on_hover_text("Click To Add Another Person To Records.")
                    .clicked()
                {
                    self.add_person_modal_open = true
                }

                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("Options", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    // ui.add_space(16.0);
                }

                ui.with_layout(egui::Layout::top_down_justified(egui::Align::RIGHT), |ui| {
                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.label(egui::RichText::new("People").size(24.0).strong());
            ui.separator();
            ui.add_space(6.0);

            for i in self.people.clone() {
                egui::Frame::new()
                    .stroke(egui::Stroke::new(1.0, Color32::GRAY))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(&i.name.to_string()).strong().heading())
                            .on_hover_text(RichText::new("Person's Name"));

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("Age:")
                                .on_hover_text(RichText::new("Person's Age In Years"));
                            ui.label(egui::RichText::new(&i.age.to_string()).strong())
                                .on_hover_text(RichText::new("Person's Age In Years"));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Has Phone:")
                                .on_hover_text(RichText::new("Dose This Person Have A Phone"));

                            if i.has_phone.clone() {
                                ui.label(
                                    egui::RichText::new("Yes")
                                        .strong()
                                        .color(egui::Color32::LIGHT_GREEN),
                                )
                                .on_hover_text(RichText::new("Person Has A Phone"));
                            } else {
                                ui.label(
                                    egui::RichText::new("No")
                                        .strong()
                                        .color(egui::Color32::LIGHT_RED),
                                )
                                .on_hover_text(RichText::new("Person Dose Not Have Phone"));
                            }
                        });

                        if ui
                            .button("Delete")
                            .on_hover_text(RichText::new("Remove Person"))
                            .clicked()
                        {
                            match &self.people.iter().position(|x| x == &i) {
                                Some(index) => {
                                    println!("Removed [{:?}]", &mut self.people.remove(*index))
                                }
                                None => {
                                    println!("Not Found");
                                }
                            }
                        }
                    });
                ui.add_space(3.0);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
