mod download;
mod drive;

fn main() {
    let tracker = download::Tracker::new();
    println!("{}", tracker.last_filename);
    // let downloads_dir = "./downloads";
    // let path = std::path::Path::new(downloads_dir);
    // if !path.exists() {
    //     std::fs::create_dir(path).expect("Failed to create directory");
    // }

    // let (url, filename) = download::get_file_url_and_filename("https://it.pk.edu.pl/studenci/na-studiach/rozklady-zajec/");
    // if !download::file_exists(&filename, path) {
    //     let file_path = download::download_file(&url, &filename, path);
    // } else {
    //     println!("File already exists: {}", path.join(&filename).to_str().unwrap());
    // }
}



