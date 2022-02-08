use std::fs;
use std::env;
use std::io;
use regex::RegexSet;
use regex::Regex;
use walkdir::WalkDir;
  

fn main() {
  let folder_name: String = get_folder_name();
  make_folder_in_current_dir(&folder_name);
  traverse_directories(&folder_name);
}


fn check_path_for_images(path: &str, folder_name: &String) {
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

  let jpg_images: Vec<_> = image_types.matches(path).into_iter().collect();
  for jpg_image in jpg_images {
    println!("This is an image {}", jpg_image);
    println!("This is the image path: {}", path);
    println!("This image should be copied to {}", folder_name);
  }
}

fn get_folder_name() -> String {
  println!("What's a good name for the photo album: ");
  let mut buffer: String = String::new();
  io::stdin()
    .read_line(&mut buffer)
    .expect("Failed to read from buffer");
  
  let whitespace: Regex = Regex::new(r"\s+").unwrap();
  let folder_name: String = whitespace.replace_all(&buffer.as_str(), "_").trim().to_string();
  return folder_name;
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

fn traverse_directories(folder_name: &String) {
  for entry in WalkDir::new(".")
    .into_iter()
    .filter_map(|e| e.ok()) {
      match entry.path().to_str() {
        Some(entry_path_str) => check_path_for_images(entry_path_str, folder_name),
        None => println!("Failed to convert path to string format"),
      }
      println!("{}", entry.path().display());
  }
}
// check photos with magic bites
// copy paths
// copy all checked photos to directory 
