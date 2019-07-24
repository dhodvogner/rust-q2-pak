extern crate binary_rw;

use binary_rw::{BinaryReader, OpenType};
use std::env;
use std::process;

fn main() {
  println!("Rust Quake/Quake2 PAK explorer");

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
  }
}

// Print the current postion
// fn dump_pos(binary_file: &mut binary_rw::BinaryReader) {
//   let pos = binary_file.get_cur_pos();
//   println!("pos: {}", pos);
// }
