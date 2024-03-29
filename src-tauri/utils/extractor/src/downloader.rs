use crate::constants::{DATA_URL, EXTRACTOR_DIR, VERSION_FILE, VERSION_REGEX};
use anyhow::Result;
use fs_extra::dir::move_dir;
use fs_extra::dir::remove;
use regex::Regex;
use std::fs::read_to_string;
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tempdir::TempDir;

#[derive(Debug)]
pub struct Downloader {
    pub version: String,
    pub latest_version: String,
}

impl Downloader {
    pub fn new(handle: &AppHandle) -> Result<Downloader> {
        let mut instance = Downloader {
            version: String::new(),
            latest_version: String::new(),
        };

        instance.get_current_version(handle)?;
        instance.get_latest_url()?;
        dbg!(&instance.is_latest());

        if !instance.is_latest() {
            instance.download_latest(handle)?;
        }

        return Ok(instance);
    }

    pub fn get_latest_url(&mut self) -> Result<()> {
        let latest_data_url = format!("{}/latest", DATA_URL);
        let client = reqwest::blocking::Client::new();
        let response = client.head(&latest_data_url).send().unwrap();
        let url = response.url().as_str();

        let final_segment = url.split('/').last().unwrap();
        assert!(
            VERSION_REGEX.is_match(final_segment),
            "Invalid version format"
        );

        self.latest_version = final_segment.to_string();

        dbg!(&self.latest_version);

        return Ok(());
    }

    pub fn get_current_version(&mut self, handle: &AppHandle) -> Result<()> {
        let version_file = handle
            .path()
            .resolve(VERSION_FILE, BaseDirectory::AppData)?;

        // check if version file exists
        if !version_file.exists() {
            return Ok(());
        }

        let version = read_to_string(version_file)?.parse::<String>()?;

        let re = Regex::new(r"(\d+\.\d+\.\d+)")?;
        assert!(re.is_match(&version), "Invalid version format");

        self.version = version;

        dbg!(&self.version);

        return Ok(());
    }

    pub fn is_latest(&self) -> bool {
        self.version == self.latest_version
    }

    pub fn download_latest(&mut self, handle: &AppHandle) -> Result<()> {
        let download_url = format!("{}/download/{}/data.zip", DATA_URL, self.latest_version);
        let temp_dir = TempDir::new(&handle.config().identifier.to_string())
            .expect("Failed to create temp dir");

        let response = reqwest::blocking::get(&download_url)?;
        let data = response.bytes()?;

        // Download data.zip
        let zip_file_path = temp_dir.path().join("data.zip");
        std::fs::write(&zip_file_path, data)
            .expect(format!("Failed to write data.zip to {:?}", zip_file_path).as_str());

        // Unzip data.zip
        let tmp_unzip_dir = temp_dir.path().join("unzip");
        let dist_folder = handle
            .path()
            .resolve(EXTRACTOR_DIR, BaseDirectory::AppData)?;
        // cleanup dist folder
        remove(&dist_folder).expect(format!("Failed to remove {:?}", dist_folder).as_str());

        // create parent directory if not exists
        if !dist_folder.exists() {
            std::fs::create_dir_all(&dist_folder)?;
        }

        let zip_file = std::fs::File::open(&zip_file_path)
            .expect(format!("Failed to open {:?}", zip_file_path).as_str());
        let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to read ZipArchive");
        archive
            .extract(&tmp_unzip_dir)
            .expect(format!("Failed to extract data.zip to {:?}", tmp_unzip_dir).as_str());
        remove(&tmp_unzip_dir.join("data/A/DofusInvoker")).expect("Failed to remove DofusInvoker");

        let options = fs_extra::dir::CopyOptions::new().content_only(true);
        let from = vec![
            tmp_unzip_dir.join("data/A"),
            tmp_unzip_dir.join("data/B"),
            tmp_unzip_dir.join("data/C"),
        ];
        for f in from {
            move_dir(&f, &dist_folder, &options).expect("Failed to move items");
        }

        // Update version file
        let version_file = handle
            .path()
            .resolve(VERSION_FILE, BaseDirectory::AppData)?;
        std::fs::write(&version_file, self.latest_version.as_bytes())
            .expect(format!("Failed to write version file to {:?}", version_file).as_str());

        self.version = self.latest_version.clone();

        temp_dir.close()?;

        return Ok(());
    }
}
