use eframe::{egui, epi};

struct MyApp {
    items: Vec<CosmeticItem>,
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Fortnite Cheat"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Fortnite Cheat");
            for item in &self.items {
                ui.horizontal(|ui| {
                    ui.label(&item.name);
                    if ui.button("Unlock").clicked() {
                        // Call unlock function here
                    }
                });
            }
        });
    }
}

fn main() {
    let items = vec![
        CosmeticItem { name: "Epic Outfit".to_string(), item_type: "outfit".to_string(), unlocked: false },
        CosmeticItem { name: "Legendary Emote".to_string(), item_type: "emote".to_string(), unlocked: false },
    ];
    let app = MyApp { items };
    eframe::run_native("Fortnite Cheat", Default::default(), Box::new(|_cc| Box::<MyApp>::default()));
}