use std::{io, fs, env};
use std::path::Path;
use regex::{Regex, RegexSet};
use walkdir::WalkDir;

fn main() {
  let folder_name: String = get_folder_name();
  make_folder_in_current_dir(&folder_name);
  traverse_directories(&folder_name);
}

fn get_folder_name() -> String {
  println!("What's a good name for the photo album: ");
  let mut buffer: String = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to read from buffer");
  
  let whitespace: Regex = Regex::new(r"\s+").unwrap();
  let folder_name: String = whitespace.replace_all(&buffer.as_str(), "_").trim().to_string();
  folder_name
}
   
fn make_folder_in_current_dir(folder_name: &String) {
  match env::current_dir() {
    Ok(folder_path) => {
      match folder_path.to_str()  {
        Some(folder_path_str) =>  println!("Made /{} folder in {}", folder_name, folder_path_str),
        None => println!("Failed to display folder path"),
      }
    },
    Err(..) => println!("Failed to read current directory"),
  }

  fs::create_dir(folder_name)
    .expect("Failed to create folder in current directory");
}

fn remove_dot(file: &str) -> &str {
  let mut new_file_name = file.chars();
  new_file_name.next();
  &new_file_name.as_str()
}

fn check_path_for_images(file_name: &str, folder_name: &String) {
  let image_types: RegexSet = RegexSet::new(&[
    r".+\.jpg",
    r".+\.jpeg",
    r".+\.png",
    r".+\.gif",
    r".+\.tiff",
    r".+\.psd",
    r".+\.pdf",
    r".+\.eps",
    r".+\.ai",
    r".+\.raw",
    r".+\.svg",
  ]).unwrap();

  if image_types.is_match(file_name) {
    
    let new_path = Path::new(".");
    let new_path_buf = new_path.join(file_name);
    
    let new_path_str: &str;
    match new_path_buf.to_str() {
      Some(s) => new_path_str = s,
      None =>  {
        println!("Failed to Create Copy Path");
        new_path_str = "FAILED PATH";
      }
    }
    
    let copy_image_path = folder_name.clone() + remove_dot(new_path_str);
    
    match fs::copy(file_name, &copy_image_path) {
      Ok(..) => println!("copied {} file", file_name),
      Err(..) => {
        println!("Failed to copy image");
        println!("Image path {}", file_name);
        println!("Image copy path {}", &copy_image_path);
      }
    }
  }
}

fn traverse_directories(folder_name: &String) {
  println!("Traversing directory and all subdirectories for images...");
  for entry in WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok()) {
      match entry.path().file_name() {
        Some(entry_path_file_name) => {
          match entry_path_file_name.to_str() {
            Some(entry_path_file_name_str) => check_path_for_images(entry_path_file_name_str, folder_name),
            None => println!("Failed to convert file name to string format"),
          }
        },
        None => println!("Failed to convert path to string format"),
      }
  }
}
//TODO: check photos with magic bites
