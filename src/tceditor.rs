use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek},
    path::PathBuf,
    ptr::write_bytes,
};

use egui::{TextEdit, Ui, scroll_area};
use tc_save_parser::{
    CircuitDataFile, CircuitDataVersion, Kind, Point, new_permament_id,
    v8::{CircuitData, Component, Wire, WireDirection, WireSegment},
};

#[derive(Default)]
pub struct TCEditor {
    // circuit: CircuitData,
    editors: Vec<TCCircuitEditor>,
}

impl TCEditor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl TCEditor {
    fn open(&mut self, path: &str) {
        self.editors.push(TCCircuitEditor::new(path));
    }
}

impl eframe::App for TCEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // if self.editors.is_empty() {
        //     let name = "autogen";
        //     let path = PathBuf::from(
        //         "C:\\Users\\ashum\\AppData\\Roaming\\godot\\app_userdata\\Turing Complete\\schematics\\component_factory\\auto",
        //     );
        //     let path = path.join(name);
        //     if let Ok(exists) = fs::exists(&path) {
        //         if !exists {
        //             fs::create_dir_all(&path).unwrap();
        //             let mut cd = CircuitData::default();
        //             cd.camera_position = Point::new(0, 0);
        //             cd.custom_id = new_permament_id();
        //             cd.components = vec![];
        //             let gc = |pos: Point| {
        //                 let mut comp = Component::default();
        //                 comp.kind = Kind::On;
        //                 comp.position = pos;
        //                 comp.permanent_id = new_permament_id();
        //                 comp
        //             };
        //             cd.components.push(gc(Point::new(0, -1)));
        //             cd.components.push(gc(Point::new(0, 1)));
        //             cd.wires.push(Wire {
        //                 color: 0,
        //                 comment: "Wire Down".into(),
        //                 start: Point::new(1, 0),
        //                 segments: vec![
        //                     WireSegment::new()
        //                         .with_direction(WireDirection::Down)
        //                         .with_length(1),
        //                     WireSegment::new(),
        //                 ],
        //             });
        //             let mut buf = Cursor::new(vec![0u8; 8192]);
        //             cd.write(&mut buf).unwrap();
        //             let len = buf.position();
        //             let fin = buf.into_inner()[..len as usize].to_vec();
        //             let cdf = CircuitDataFile {
        //                 version: 8,
        //                 data: fin,
        //             };
        //             let mut fh = File::create(path.join("circuit.data")).unwrap();
        //             cdf.write(&mut fh).unwrap();
        //         }
        //     }
        //     self.open(path.join("circuit.data").to_str().unwrap());
        // }
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // file_menu_button(ui);
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let files = rfd::FileDialog::new()
                            .add_filter("circuit.data", &["data"])
                            .pick_file();
                        if files.is_none() {
                            return;
                        }
                        let path = files.unwrap().to_str().unwrap().to_string();
                        self.open(&path);
                    }
                    // if ui.button("Save").clicked() {
                    //     // let path = files.unwrap().to_str().unwrap().to_string();
                    //     // let mut editor = TCCircuitEditor::new(&path);
                    //     // editor.save(&path);
                    // }
                })
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            for editor in &mut self.editors {
                editor.ui(ui);
            }
        });
    }
}

pub struct TCCircuitEditor {
    circuit: CircuitData,
    path: String,
}

impl TCCircuitEditor {
    pub fn new(path: &str) -> Self {
        let circuitfile = match CircuitDataFile::load(&path) {
            Ok(circuitfile) => circuitfile,
            Err(err) => {
                match err {
                    tc_save_parser::Error::UnsupportedVersion(version, data) => {
                        eprintln!("Unsupported version: {} {:?}", version, data);
                        fs::write("uncompressed.data", data).unwrap();
                    }
                    tc_save_parser::Error::Binrw(err) => {
                        let data = CircuitDataFile::debug_dump(path).unwrap();
                        fs::write("uncompressed.data", data).unwrap();
                        eprintln!("Error: {:?}", err)
                    }
                    _ => eprintln!("Error: {:?}", err),
                }
                std::process::exit(1);
            }
        };
        let circuit = match circuitfile.circuit {
            CircuitDataVersion::V8(circuit) => circuit,
            _ => panic!("Unsupported version"),
        };
        let mut s = Self {
            circuit,
            path: path.to_string(),
        };
        s.init();
        s
    }

    pub fn save(&self, path: &str) {
        let cdf = CircuitDataFile {
            version: 8,
            circuit: CircuitDataVersion::V8(self.circuit.clone()),
        };
        cdf.save(path).unwrap();
    }
    fn ui(&mut self, ui: &mut Ui) {
        for component in self.circuit.components.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(format!("Kind {:?}", component.kind));
                ui.label(format!("Pos {:?}", component.position));
                ui.label(format!("Perm ID {:?}", component.permanent_id));
                if (component.custom_string.value.len() > 0) {
                    ui.label(format!("Custom {:?}", component.custom_string));
                }
                if (component.settings.len() > 0) {
                    ui.label(format!("Settings {:?}", component.settings));
                }
                ui.label(format!("Word Size {:?}", component.word_size));
                if (component.kind.is_custom()) {
                    ui.label(format!("Custom {:?}", component.custom));
                }
                if (component.buffer_size > 0) {
                    ui.label(format!("Buffer Size {:?}", component.buffer_size));
                }
                if (component.kind.has_linked_components()) {
                    ui.label(format!(
                        "Linked Components {:?}",
                        component.linked_components
                    ));
                }
                if (component.kind == Kind::Assembler) {
                    ui.label(format!("Assembler {:?}", component.assembler_info));
                }
            });
        }
        for wire in self.circuit.wires.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(format!("{:?}", wire));
            });
        }
        //self.circuit.wires
    }

    fn init(&mut self) {
        // self.circuit.camera_position = Point::new(0, 0);
        // self.circuit.components = vec![];
        // let mut comps = vec![];
        // let mut comp = Component::default();
        // comp.kind = Kind::On;
        // comp.position = Point::new(0, -1);
        // comp.permanent_id = new_permament_id();
        // comps.push(comp);
        // let mut comp = Component::default();
        // comp.kind = Kind::Off;
        // comp.position = Point::new(0, 1);
        // comp.permanent_id = new_permament_id();
        // comps.push(comp);

        // self.circuit.wires.push(Wire {
        //     color: 0,
        //     comment: "Wire DOwn".into(),
        //     start: Point::new(1, 0),
        //     segments: vec![
        //         WireSegment::new()
        //             .with_direction(WireDirection::Down)
        //             .with_length(10),
        //         WireSegment::new(),
        //     ],
        // });

        // self.circuit.components = comps;

        // self.save(&self.path);
    }
    // fn ui2(&mut self, ui: &mut Ui) {
    //     ui.heading("Turing Complete Circuit Editor");
    //     ui.label(format!("Path: {}", &self.path));
    //     ui.separator();
    //     ui.label("Components:");
    //     scroll_area::ScrollArea::vertical().show(ui, |ui| {
    //         for component in self.circuit.components.iter_mut() {
    //             ui.label(format!(
    //                 "Kind: {:?} ({:?})",
    //                 component.kind,
    //                 Kind::from(component.kind)
    //             ));
    //             ui.label(format!("Position: {:?}", component.position));
    //             ui.horizontal(|ui| {
    //                 ui.label(format!("Word Size: "));
    //                 let mut str = component.word_size.to_string();
    //                 let resp = ui.add(TextEdit::singleline(&mut str));
    //                 if resp.changed() {
    //                     let num = str.parse();
    //                     match num {
    //                         Ok(num) => {
    //                             component.word_size = num;
    //                         }
    //                         Err(_) => {
    //                             component.word_size = i64::MIN;
    //                         }
    //                     }
    //                 }
    //             });
    //         }
    //     });
    // }
}
