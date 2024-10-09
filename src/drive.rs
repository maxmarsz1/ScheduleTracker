use drive_v3::{objects::{File, UploadType}, Credentials, Drive};

pub struct CustomDrive{
    scopes: Vec<&'static str>,
    credentials: Credentials,
    drive: Drive
}

impl CustomDrive{
    pub fn new(credentials_path: &str, client_secret_path: &str, scopes: Option<Vec<&'static str>>) -> CustomDrive{
        let scopes = scopes.unwrap_or(vec!["https://www.googleapis.com/auth/drive.metadata", "https://www.googleapis.com/auth/drive"]);
        let mut credentials;
        if std::path::Path::new(credentials_path).exists(){
            println!("Found credentials file: {}", credentials_path);
            credentials = Credentials::from_file(credentials_path, &scopes).unwrap();
            if !credentials.are_valid(){
                credentials.refresh().expect("Failed to refresh credentials");
                credentials.store(credentials_path).expect("Failed to store credentials");
            }
        } else {
            println!("No credentials file found: {}", credentials_path);
            credentials = Credentials::from_client_secrets_file(client_secret_path, &scopes).unwrap();
            credentials.store(credentials_path).expect("Failed to store credentials");
        }
        let drive = Drive::new(&credentials);

        CustomDrive{scopes, credentials, drive}
    }

    // pub fn new_from_stored_credentials(credentials_path: &str, scopes: Option<Vec<&'static str>>) -> CustomDrive{
    //     let scopes = scopes.unwrap_or(vec!["https://www.googleapis.com/auth/drive.metadata", "https://www.googleapis.com/auth/drive"]);
    //     let mut stored_credentials = Credentials::from_file(credentials_path, &scopes).expect("Failed to load credentials");
    //     if !stored_credentials.are_valid(){
    //         stored_credentials.refresh().expect("Failed to refresh credentials");
    //         stored_credentials.store(credentials_path).expect("Failed to store credentials");
    //     }
    //     let drive = Drive::new(&stored_credentials);
    //     CustomDrive{scopes, credentials: stored_credentials, drive}
    // }
    
    pub fn list_files(&self){
        let drive = &self.drive;
        let file_list = drive.files.list().execute().expect("Failed to list files");
        if let Some(files) = file_list.files {
            for file in &files {
                println!("{}", file);
            }
        }
    }

    pub fn upload_file(&self, filename: &str){
        let drive = &self.drive;
        let new_file = drive.files.create()
        .upload_type(UploadType::Multipart)
        .metadata(File{
            name: Some(filename.to_string()),
            mime_type: Some("application/vnd.ms-excel".to_string()),
            ..Default::default()
        })
        .content_source("./downloads/".to_string() + filename)
        .execute()
        .unwrap();
    }
    
    pub fn update_file(&self, file_id: &str, filename: &str){
        let drive = &self.drive;
        let mut metadata = drive.files.get(file_id).execute().expect("Failed to get file");
        metadata.id = None;
        metadata.name = Some(filename.to_string());
        drive.files.update(file_id)
        .upload_type(UploadType::Multipart)
        .metadata(metadata)
        .content_source("./downloads/".to_string() + filename)
        .execute()
        .expect("Failed to update file");
    }
}