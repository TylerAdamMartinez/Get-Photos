use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io;
use regex::RegexSet;
use regex::Regex;

fn main() {
  let folder_name: String = get_folder_name();
  make_folder_in_current_dir(folder_name);
  
  println!("Finding images files...");
  /* 
  let mut ancestors = Path::new("../foo/bar").ancestors();
  assert_eq!(ancestors.next(), Some(Path::new("../foo/bar")));
  assert_eq!(ancestors.next(), Some(Path::new("../foo")));
  assert_eq!(ancestors.next(), Some(Path::new("..")));
  assert_eq!(ancestors.next(), Some(Path::new("")));
  assert_eq!(ancestors.next(), None);
  */

  let path = Path::new(".");
  for entry in fs::read_dir(path).expect("Unable to list") {
      let entry = entry.expect("unable to get entry");
      println!("{}", entry.path().display());

      check_path_for_images(entry.path());

      if !entry.path().is_file() {
        for sub_entry in fs::read_dir(entry.path()).expect("Unable to list") {
          match sub_entry {
            Ok(sub_entry) =>  {
              println!("{}", sub_entry.path().display());
              check_path_for_images(sub_entry.path());
            },
            Err(..) => println!("unable to get sub entry"),
          }
        }
      }
  }
}


fn check_path_for_images(path: PathBuf) {
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
  ]).unwrap();

  match path.to_str() {
    None => println!("Image not found"),
    Some(path_str) => {
      let jpg_images: Vec<_> = image_types.matches(path_str).into_iter().collect();
      for jpg_image in jpg_images {
        println!("This is an image {}", jpg_image);
      }
    },
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
   
fn make_folder_in_current_dir(folder_name: String) {
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
// check photos with magic bites
// copy paths
// copy all checked photos to directory 
