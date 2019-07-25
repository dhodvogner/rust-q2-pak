extern crate binary_rw;

use binary_rw::{BinaryReader, OpenType};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

fn main() {
  println!("Rust Quake/Quake2 PAK parser");

  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("Please specify the filename as the first argument!");
    process::exit(1);
  }

  let filename = &args[1];
  println!("Filename: {}", filename);

  let mut binary_file = BinaryReader::new(filename, OpenType::Open);

  // Reading the header (64 bytes)
  let read_value = binary_file.read_bytes(4);
  let id = String::from_utf8(read_value).unwrap();

  if id != "PACK" {
    println!("Unknown file format!");
    process::exit(1);
  }

  let offset = binary_file.read_i32();
  let size = binary_file.read_i32();
  let number_of_files = size / 64;

  println!("=== Header ===");
  println!("ID: {}\n  Offset: {}\n  Size: {}", id, offset, size);
  println!("Number of files: {}", number_of_files);
  println!("==============");

  binary_file.seek_to(offset as u64);
  // Read all file entries
  for x in 0..number_of_files {
    // Reading a file entry (64 bytes)
    let read_value = binary_file.read_bytes(56);
    let file_name = String::from_utf8(read_value).unwrap();
    let offset = binary_file.read_i32();
    let size = binary_file.read_i32();

    println!(
      "#{} File Name: '{}' (Offset: {} Size: {})",
      x, file_name, offset, size
    );
    export_file(&mut binary_file, file_name, offset, size);
  }
}

fn export_file(
  binary_file: &mut binary_rw::BinaryReader,
  file_name: String,
  offset: i32,
  size: i32,
) {
  let out_file_name = String::from(&format!("out/{}", file_name));
  let out_file_name = out_file_name.trim_matches(char::from(0));

  let prev_pos = binary_file.get_cur_pos();
  // Read file content
  binary_file.seek_to(offset as u64);
  let data = binary_file.read_bytes(size as u64);
  binary_file.seek_to(prev_pos);

  let path = Path::new(&out_file_name);
  let display = path.display();
  let parent = path.parent().unwrap();
  let parent_display = parent.display();

  match fs::create_dir_all(&parent) {
    Err(why) => panic!(
      "Couldn't create folders {}: {}",
      parent_display,
      why.description()
    ),
    Ok(_) => println!("Successfully created folders {}", parent_display),
  };;

  let mut file = match File::create(&path) {
    Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
    Ok(file) => file,
  };

  match file.write_all(&data) {
    Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
    Ok(_) => println!("Successfully wrote to {}", display),
  }
}
