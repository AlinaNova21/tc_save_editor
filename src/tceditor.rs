use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek},
    path::PathBuf,
    ptr::write_bytes,
};

use egui::{TextEdit, Ui, scroll_area};
use tc_save_parser::{
    CircuitDataFile, CircuitDataVersion, Kind, Point, new_permament_id,
    v9::{CircuitData, Component, Wire, WireDirection, WireSegment},
};
use yosys_netlist_json::Netlist;

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
        if self.editors.is_empty() {
            if let Some(path) = generate() {
                self.open(&path);
            }
        }
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
            CircuitDataVersion::V9(circuit) => circuit,
            CircuitDataVersion::Unknown(data) => {
                fs::write("uncompressed.data", data).unwrap();
                panic!("Unsupported version: {}", circuitfile.version)
            }
            _ => {
                panic!("Unsupported version: {}", circuitfile.version)
            }
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
            version: 9,
            circuit: CircuitDataVersion::V9(self.circuit.clone()),
        };
        cdf.save(path).unwrap();
    }
    fn ui(&mut self, ui: &mut Ui) {
        scroll_area::ScrollArea::vertical().show(ui, |ui| {
            let sort_button = ui.button("Sort Components Alphabetically");
            if sort_button.clicked() {
                alphanumeric_sort::sort_slice_by_str_key(&mut self.circuit.components, |a| {
                    &a.custom_string.value
                });
            }
            let save_button = ui.button("Save");
            if save_button.clicked() {
                self.save(&self.path);
            }
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
                });
                // if (component.kind == Kind::Assembler) {
                //     ui.label(format!("  Assembler {:?}", component.watched_components));
                // }
            }
            for wire in self.circuit.wires.iter_mut() {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", wire));
                });
            }
        });
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
        // let id = self.circuit.components[3].permanent_id;
        // // self.circuit.components[0]
        // //     .linked_components
        // //     .linked_components[0] = id;
        // self.circuit.components[0]
        //     .linked_components
        //     .linked_components = vec![id];

        // self.save(&self.path);
        // let mut circuit = CircuitData::default();
        // circuit.custom_id = new_permament_id();

        // let mut fh = File::open("input.json").unwrap();
        // let nl = Netlist::from_reader(&mut fh).unwrap();
        // let module = nl.modules.get("ibex_load_store_unit").unwrap();

        // for (name, port) in &module.ports {
        //     let mut comp = Component::default();
        //     comp.kind = Kind::On;
        //     comp.position = Point::new(0, 0);
        //     comp.permanent_id = new_permament_id();
        //     comp.custom_string = name.clone().into();
        //     circuit.components.push(comp);
        // }
    }
}

fn generate() -> Option<String> {
    None
    // let inputs = [
    //     "lsu_req",
    //     "data_pmp_err",
    //     "lsu_we",
    //     "data_gnt",
    //     "split_misaligned_access",
    // ];
    // let outputs = [
    //     "data_req",
    //     "pmp_err",
    //     "perf_load",
    //     "perf_store",
    //     "ctrl_update",
    //     "addr_update",
    //     "handle_misaligned",
    //     "lsm_fsm",
    // ];
    // let name = "autogen";
    // let path = PathBuf::from(
    //     "C:\\Users\\ashum\\AppData\\Roaming\\godot\\app_userdata\\Turing Complete\\schematics\\component_factory\\auto",
    // );
    // let path = path.join(name);
    // if let Ok(exists) = fs::exists(&path) {
    //     // if !exists || true {
    //     if true {
    //         fs::create_dir_all(&path).unwrap();
    //         let mut cd = CircuitData::default();
    //         cd.camera_position = Point::new(0, 0);
    //         cd.custom_id = new_permament_id();
    //         cd.components = vec![];
    //         let gen_input = |name: &str, pos: Point| {
    //             let mut comp = Component::default();
    //             comp.kind = Kind::CcInput;
    //             comp.position = pos;
    //             comp.permanent_id = new_permament_id();
    //             comp.custom_string = name.into();
    //             comp.word_size = i64::MIN;
    //             comp
    //         };
    //         let gen_output = |name: &str, pos: Point| {
    //             let mut comp = Component::default();
    //             comp.kind = Kind::CcOutput;
    //             comp.position = pos;
    //             comp.permanent_id = new_permament_id();
    //             comp.custom_string = name.into();
    //             comp
    //         };

    //         for (i, name) in inputs.iter().enumerate() {
    //             let comp = gen_input(name, Point::new(0, (i as i16) * 8));
    //             cd.components.push(comp);
    //         }
    //         for (i, name) in outputs.iter().enumerate() {
    //             let comp = gen_output(name, Point::new(24, (i as i16) * 8));
    //             cd.components.push(comp);
    //         }

    //         let cdf = CircuitDataFile {
    //             version: 8,
    //             circuit: CircuitDataVersion::V8(cd),
    //         };
    //         let path = path.join("circuit.data");
    //         cdf.save(path.to_str().unwrap()).unwrap();
    //     }
    // }
    // Some(path.join("circuit.data").to_str().unwrap().to_string())
}
