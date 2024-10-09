use crate::drive::{self, CustomDrive};
use std::{collections::HashMap, fs::File, io};

use chrono::NaiveDate;

pub struct Tracker{
    pub last_filename: String,
    pub drive: CustomDrive
}

impl Tracker{
    pub fn new() -> Tracker{
        let drive = CustomDrive::new("./credentials.json", "./client_secrets.json", None);
        let downloaded_files = std::fs::read_dir("./downloads")
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect::<Vec<String>>();
        println!("Downloaded files: {:?}", downloaded_files);

        let file_url = get_file_url("https://it.pk.edu.pl/studenci/na-studiach/rozklady-zajec/");
        let filename = file_url.split('/').last().expect("Failed to find filename");
        let last_downloaded_filename = downloaded_files.iter().find(|downloaded_filename| *downloaded_filename == filename);
        match last_downloaded_filename {
            Some(last_filename) => Tracker{last_filename:last_filename.clone(), drive},
            None => {
                download_file(&file_url, filename, std::path::Path::new("./downloads"));
                Tracker{last_filename: filename.to_string(), drive}
            },
        }
    }

    // pub fn download_if_new_available(&self) {
    //     let url = get_file_url("https://it.pk.edu.pl/studenci/na-studiach/rozklady-zajec/");
    //     let filename = url.split('/').last().expect("Failed to find filename");
    //     if filename != self.last_filename {
    //         download_file(&url, filename, &std::path::Path::new("./downloads"));
    //         self.last_filename = filename.to_string();
    //         // self.drive.upload_file(&filename);
    //     }
    // }





}

pub fn get_file_url(url: &str) -> String {
    let response = reqwest::blocking::get(url).expect("Failed to connect to server");
    let body = response.text().unwrap();
    let text_match = "class=\"bst-filelist__item-link\" href=\"";
    let start_position = body.find(text_match).expect("Failed to find text match");
    let end_position = body[start_position+text_match.len()..].find("\"").unwrap();

    let url = &body[start_position + text_match.len()..start_position + text_match.len() + end_position];
    println!("Found url: {}", url);

    url.to_string()
}

pub fn download_file(url: &str, filename: &str, path: &std::path::Path){
    println!("Downloading file: {}", filename);
    let response = reqwest::blocking::get(url).expect("Failed to connect to server");
    let body = response.bytes().expect("Failed to read response");
    let mut out = File::create(path.join(filename)).expect("Failed to create file");
    io::copy(&mut body.as_ref(), &mut out).expect("Failed to write file");
    println!("File downloaded: {}", path.join(filename).to_str().unwrap());
}