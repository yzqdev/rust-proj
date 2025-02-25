#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::Arc;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // 创建 FontDefinitions 对象
            let mut font_definitions = FontDefinitions::default();
            let mut ctx = cc.egui_ctx.clone();
            // 加载 Microsoft YaHei 字体
            // 在 Windows 上，字体通常位于 C:\Windows\Fonts\msyh.ttc
            let font_data = FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc"));
            // 将 Microsoft YaHei 字体设置为默认字体
            font_definitions
                .font_data
                .insert("msyh".to_owned(), Arc::new(font_data));
            // 设置默认字体
            font_definitions
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "msyh".to_owned());
            // 设置字体定义到 egui 上下文
            ctx.set_fonts(font_definitions);
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "如何评价桌面与嵌入式gui开发库slint".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!("assets/ferris.png"));
        });
    }
}
