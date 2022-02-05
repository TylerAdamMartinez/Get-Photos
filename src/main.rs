use std::fs;
use std::path::Path;
use std::io;

fn main() {
    println!("What's a good name for the photo album: ");
    let mut buffer: String = String::new();
    io::stdin()
      .read_line(&mut buffer)
      .expect("Failed to read from buffer");

    let folder_name: String = buffer.trim().to_string();
    let folder_path: String = String::from("pwd here..");

    println!("Made /{} folder in {}", folder_name, folder_path);
    let image_types: Vec<String> = vec![
      String::from("*.jpg"),
      String::from("*.jpeg"),
      String::from("*.png"),
      String::from("*.gif"),
      String::from("*.tiff"),
      String::from("*.psd"),
      String::from("*.pdf"),
      String::from("*.eps"),
      String::from("*.ai"),
      String::from("*.raw"),
    ];

    // find all photos
    println!("Finding images files...");
    
    let path = Path::new(".");
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        println!("{}", entry.path().display());
    }
}

// check photos with magic bites
// copy paths
// copy all checked photos to directory 
