//use std::{process, fs::File, io::Write};
//Quick and dirty sine table generator of asm projecys
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui::{Vec2};
use egui::plot::{Plot, PlotPoints, Points};

fn main() {
    let _options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(300.0 as f32, 430 as f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        run_and_return: false,
    };
    
    eframe::run_native(
        "Front end demo",
        _options,
        Box::new(|_cc| Box::new(SineApp::default())),
    );
}

struct SineApp {
    asmlabel: String,
    amplitude: u8,
    npoints: u16,
    points: Vec<i16>,
    code: String,
}

impl Default for SineApp {
    fn default() -> Self {
        Self {
            asmlabel: "sine".to_owned(),
            npoints: 0,
            amplitude: 30,
            points: vec![], 
            code: "".to_owned(),           
        }
    }
}

impl SineApp{
    pub fn to_code(&mut self){
        self.code = "".to_owned();
        self.code += &self.asmlabel;
        self.code += ":";

        let mut i = 0;
        for p in &self.points{
            if i ==0 || i % 8 == 0 {
                self.code += "\n    dc.b ";
                self.code += &p.to_string();
            }
            else{
                self.code += ", ";
                self.code += &p.to_string();
            }
            i+=1;
        }
        self.code += "\nend";
        self.code += &self.asmlabel;
    }
}

impl eframe::App for SineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.points.clear();
        let sin: PlotPoints = (0..self.npoints).map(|i| {
            let x = i as f64;
            let m = i as f32 / self.npoints as f32 * (2.0 * std::f32::consts::PI);
            let p = f32::sin(m) * self.amplitude as f32;
            self.points.push(p as i16);
            [x, p as f64]        
        }).collect();
        let points = Points::new(sin);

        self.to_code();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sine creator");
            ui.vertical(|ui| {
                ui.label("ASM Label: ");
                ui.text_edit_singleline(&mut self.asmlabel);
                ui.label("n_points: ");
                ui.add(egui::Slider::new(&mut self.npoints, 0..=255).text("# points"));
                ui.add(egui::Slider::new(&mut self.amplitude, 0..=255).text("amplitude"));
                Plot::new("sine").view_aspect(2.0).width(290.0).height(200.0).allow_drag(false).show(ui, |plot_ui| plot_ui.points(points));
                egui::ScrollArea::vertical().max_height(75.0).show(ui, |ui| {
                    ui.text_edit_multiline(&mut self.code);
                });
                let btn_export = ui.button("Export");
                if btn_export.clicked(){

                }
            });
        });
    }
}