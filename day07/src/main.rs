use std::fs;
//use std::collections::HashSet;
// use crate::Actions::{CMD, NewFile, NewFolder};

fn main() {
    let _: String = fs::read_to_string("./day07/src/input.txt").expect("Could not open input file");
    // let part1 = run_commands(commands);
    
}

// So close but had to solve this in Kotlin instead
// 
// fn run_commands(commands: String) -> i32 {
//     // Add root folder
//     let mut directories: Vec<Directory> = vec![Directory { name: "/".to_string(), sub_folders: HashSet::new(), parent_folder: 0, files: HashSet::new() }];
//     let mut current_dir_idx = 0;
// 
//     for command in commands.lines() {
//         let cmd: Vec<&str> = command.split_whitespace().collect();
//         let action = match cmd[0] {
//             "$" => CMD,
//             "dir" => NewFolder,
//             _ => NewFile
//         };
// 
// 
//         match action {
//             NewFolder => {
//                 let new_dir = Directory { id: Uuid::new_v4(), name: cmd[1].to_string(), sub_folders: HashSet::new(), parent_folder: current_dir_idx, files: HashSet::new() };
//                 directories.push(new_dir);
//             }
//             NewFile => {
//                 let size = cmd[1].parse::<i32>().unwrap();
//                 let name = cmd[2].to_string();
//                 let new_file = File { name, size };
//                 directories[current_dir_idx].files.insert(new_file);
//             }
//             CMD => {
//                 let current_dir = &directories[current_dir_idx];
//                 let command = cmd[1];
//                 eprintln!("cmd = {:?}", cmd);
//             
//                 if command == "cd" {
//                     current_dir_idx = match cmd[2] {
//                         ".." => { current_dir.parent_folder }
//                         &_ => { current_dir.sub_folders.iter().position(|x| x.name == cmd[2]).unwrap() }
//                     };
//                 }
//             }
//         }
//     }
//     0
// }
// 
// pub enum Actions {
//     NewFolder,
//     NewFile,
//     CMD,
// }
// 
// struct Directory {
//     name: String,
//     sub_folders: HashSet<Directory>,
//     parent_folder: usize,
//     files: HashSet<File>,
// }
// 
// impl StorageTreeCalc for Directory {
//     fn total_used_storage_size(&self) -> i32 {
//         let mut total_size = 0;
//         // Calc file total
//         for file in self.files.iter() {
//             total_size += file.size;
//         }
//         // Calc all sub folder file sizes
//         for sub_folder in self.sub_folders.iter() {
//             total_size += sub_folder.total_used_storage_size()
//         }
//         total_size
//     }
// }
// 
// #[derive(Eq, Hash, PartialEq)]
// struct File {
//     name: String,
//     size: i32,
// }
// 
// trait StorageTreeCalc {
//     fn total_used_storage_size(&self) -> i32;
// }
