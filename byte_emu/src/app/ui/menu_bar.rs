use crate::app::{ByteEmuApp, FileProcesserMessage, State};

impl ByteEmuApp {
    pub fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.ui_menu_bar(ui);
        });
    }

    fn ui_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                use FileProcesserMessage::*;

                if ui.button("Load binary program").clicked() {
                    self.file_processer
                        .read(|name, data| BinaryFile((name, data)));
                    ui.close_menu();
                }
                if ui.button("Load source file").clicked() {
                    self.file_processer
                        .read(|name, data| SourceFile((name, data)));
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Reset GUI state").clicked() {
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
                if ui.button("Reset everything").clicked() {
                    self.state = State::default();
                    ui.ctx().memory_mut(|mem| *mem = Default::default());
                    ui.close_menu();
                }
            });

            ui.menu_button("Tools", |ui| {
                if ui.button("Code Editor").clicked() {
                    self.state.is_code_editor_open = true;
                    ui.close_menu();
                }

                if ui.button("Frame History").clicked() {
                    self.state.is_frame_history_open = true;
                    ui.close_menu();
                }
            });

            if ui.button("About").clicked() {
                self.state.is_about_open = true;
            }
        });
    }
}
