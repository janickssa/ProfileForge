use eframe::egui;
use crate::lib::{
    Config,
    load_config,
    save_config,
};

pub fn launch_gui() {
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

struct MyEguiApp {
    text_input: String,
    checkbox_value: bool,
    click_count: u32,
}




impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            text_input: String::new(),
            checkbox_value: false,
            click_count: 0,
        }
    }
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

// impl eframe::App for MyEguiApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Hello World!");
//             ui.label("This is a simple egui application.");

//             ui.text_edit_singleline(&mut self.text_input);
//             ui.checkbox(&mut self.checkbox_value, "Enable something");

//             if ui.button("Click me").clicked() {
//                 self.click_count += 1;
//                 println!("Button clicked! Text was: {}", self.text_input);
//             }

//             ui.label(format!("Clicked {} times", self.click_count));
//         });
//     }
// }



impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut config = load_config("config.json");
            ui.heading("ProfileForge configuration UI");
            ui.label(format!("Chrome installation directory: {}", config.chrome_path));
            ui.label(format!("Profiles path: {}", config.profiles_directory))



            if ui.button("Save").clicked() {
                println!("Saving configuration...");

            }

        });
    }
}