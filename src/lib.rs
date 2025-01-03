use tceditor::TCEditor;

mod tceditor;

pub fn run() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "TCEditor",
        native_options,
        Box::new(|cc| Ok(Box::new(TCEditor::new(cc)))),
    )
    .unwrap();
}
