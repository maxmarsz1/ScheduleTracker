use crate::drive::CustomDrive;
use std::{fs::File, io};
use rand::Rng;

pub struct Tracker{
    pub last_filename: String,
    pub drive: CustomDrive
}

impl Tracker{
    pub fn new() -> Tracker{
        let drive = CustomDrive::new("./credentials.json", "./client_secrets.json", None);

        let mut entries = std::fs::read_dir("./downloads")
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .collect::<Vec<std::fs::DirEntry>>();
        entries.sort_by(|a, b| b.metadata().unwrap().modified().unwrap().cmp(&a.metadata().unwrap().modified().unwrap().clone()));
        let last_modified_file = entries.first().expect("No files in ./downloads directory").file_name().into_string().unwrap();

        println!("Found latest schedule: {}", last_modified_file);

        Tracker{last_filename: last_modified_file, drive}
    }

    pub fn run(&mut self){
        loop {
            self.download_if_new_available();
            let sleep_time = rand::thread_rng().gen_range(1..100);
            std::thread::sleep(std::time::Duration::from_secs(300 + sleep_time as u64));
        }
    }

    pub fn download_if_new_available(&mut self) {
        let url = get_file_url("https://it.pk.edu.pl/studenci/na-studiach/rozklady-zajec/");
        let filename = url.split('/').last().expect("Failed to find filename");
        if filename != self.last_filename {
            println!("Found new schedule");
            download_file(&url, filename, std::path::Path::new("./downloads"));
            if self.drive.update_file("14IfrHlUTIMlLfGinlIz6cSvhlnLuW6P_", filename).is_ok(){
                self.last_filename = filename.to_string();
                println!("Updated new schedule");
            } else {
                println!("Failed to update schedule");
            }
        } else {
            println!("Schedule up to date");
        }
    }
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