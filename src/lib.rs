use std::{
    fs::{self, File},
    io::Cursor,
    path,
};

use binrw::{BinRead, BinWrite};
use directories::BaseDirs;
use tc::{CircuitDataFile, Kind, v8::CircuitData};
use tceditor::TCEditor;

mod tc;
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

// pub fn run2() {
//     // let level = "architecture";
//     // let save_name = "RV32I/TEST";
//     // let level = "component_factory";
//     // let save_name = "RV32I/V2/LSU";
//     let bd = BaseDirs::new().unwrap();
//     let basepath = bd.config_dir();
//     let basepath = path::PathBuf::from(basepath)
//         .join("godot")
//         .join("app_userdata")
//         .join("Turing Complete")
//         .join("schematics");
//     // let appdata = std::env::var("APPDATA").unwrap();
//     // let path = format!(
//     //     "{}/godot/app_userdata/Turing Complete/schematics/{}/{}/circuit.data",
//     //     appdata, level, save_name,
//     // );
//     use rfd::FileDialog;
//     println!("basepath: {:?}", basepath);
//     let files = FileDialog::new()
//         .add_filter("circuit.data", &["data"])
//         .set_directory(basepath)
//         .pick_file();
//     if files.is_none() {
//         return;
//     }
//     let path = files.unwrap().to_str().unwrap().to_string();
//     fs::copy(&path, &path.replace(".data", ".data.bak")).unwrap();
//     let opath = path.clone().replace("circuit.data", "circuit.data");
//     let mut fh = File::open(&opath).unwrap();
//     let circuit_data_file = CircuitDataFile::read(&mut fh).unwrap();
//     // fs::write("decompressed.data", &circuit_data_file.data).unwrap();
//     let mut circuit_data = CircuitData::read(&mut Cursor::new(&circuit_data_file.data)).unwrap();
//     // println!("{:?}", circuit_data);

//     let mut components = circuit_data.components.clone();
//     for i in 0..components.len() {
//         let comp = &mut components[i];
//         if comp.word_size >= 0 {
//             continue;
//         }
//         println!("Found {:?} with 'x' word size", Kind::from(comp.kind));
//         match Kind::from(comp.kind) {
//             Kind::Splitter2 => {
//                 comp.word_size = 42;
//                 println!("Patched word_size");
//             }
//             Kind::Splitter4 => {
//                 comp.word_size = 42;
//                 println!("Patched word_size");
//             }
//             Kind::Splitter8 => {
//                 comp.word_size = 42;
//                 println!("Patched word_size");
//             }
//             _ => {}
//         }
//         println!("");
//         components[i] = comp.clone();
//     }
//     circuit_data.components = components;

//     // let data = fs::read("decompressed.data").unwrap();
//     // let circuit_data = CircuitDataFile { version: 7, data };
//     // // circuit_data.write(&mut fh).unwrap();
//     // fh.seek(SeekFrom::Start(0)).unwrap();
//     let mut fh = File::options()
//         .write(true)
//         .create(true)
//         .open("decompressed.data")
//         .unwrap();
//     circuit_data.write(&mut fh).unwrap();
//     let data = fs::read("decompressed.data").unwrap();
//     let newpath = path.replace("circuit.data", "circuit.data.new");
//     let mut fh = File::options()
//         .write(true)
//         .create(true)
//         .open(&newpath)
//         .unwrap();

//     // fs::write("decompressed.data", &circuit_data_file.data).unwrap();
//     let circuit_data_file = CircuitDataFile {
//         version: circuit_data_file.version,
//         data,
//     };
//     circuit_data_file.write(&mut fh).unwrap();
//     CircuitData::read(&mut Cursor::new(circuit_data_file.data)).unwrap();

//     fs::rename(&newpath, &path).unwrap();
// }
