use super::download::{
    Download, DownloadUrl, DownloadUrlHttp, DownloadUrlTorrentMagnet, Downloaded, DownloadedFile,
};
use crate::{worker::models, BitflowError};
use std::io::prelude::*;
use std::process::Command;
use std::{fs, io};
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Downloader {
    api_client: Option<reqwest::Client>,
    api_endpoint: Option<String>,
}

impl Downloader {
    pub fn new(api_client: Option<reqwest::Client>, api_endpoint: Option<String>) -> Downloader {
        return Downloader {
            api_client,
            api_endpoint,
        };
    }

    #[allow(clippy::unused_io_amount)]
    pub fn download(&self, download: Download) -> Result<Downloaded, BitflowError> {
        // create directory
        let directory = format!("downloads/{}", download.id);
        fs::create_dir(&directory)?;

        // download
        match &download.url {
            DownloadUrl::Http(ref url) => self.download_http(download.id, url, &directory)?,
            DownloadUrl::TorrentMagnet(ref url) => {
                self.download_torrent_magnet(download.id, url, &directory)?
            }
        }

        // walk directory to retrieve downlaoded files
        let mut files: Vec<DownloadedFile> = Vec::new();
        let mut total_size = 0u64;
        for entry in WalkDir::new(directory)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let content_type = {
                // read first 512 bytes to detect content type
                let mut contents = [0u8; 512];
                let mut file = fs::File::open(entry.path())?;
                file.read(&mut contents)?;
                mimesniff::detect_content_type(&contents)
            };

            let size = entry.metadata()?.len();
            total_size += size as u64;

            if size > download.max_size || total_size > download.max_size {
                return Err(BitflowError::Internal(
                    "Files too large, aborting.".to_string(),
                ));
            }

            let file = DownloadedFile {
                bitflow_id: uuid::Uuid::new_v4(),
                name: entry
                    .path()
                    .file_name()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no filename"))?
                    .to_str()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no filename"))?
                    .to_string(),
                path: format!("{}", entry.path().display()),
                size,
                type_: content_type.to_string(),
            };
            files.push(file);
        }

        return Ok(Downloaded { download, files });
    }

    fn download_http(
        &self,
        _dl_id: uuid::Uuid,
        dl: &DownloadUrlHttp,
        directory: &str,
    ) -> Result<(), BitflowError> {
        let mut response = reqwest::get(&dl.url)?;
        let mut dest = {
            let fname = response
                .url()
                .path_segments()
                .and_then(std::iter::Iterator::last)
                .and_then(|name| if name.is_empty() { None } else { Some(name) })
                .unwrap_or("index.html");

            let fname = format!("{}/{}", directory, fname);
            fs::File::create(fname)?
        };
        io::copy(&mut response, &mut dest)?;
        return Ok(());
    }

    fn download_torrent_magnet(
        &self,
        dl_id: uuid::Uuid,
        dl: &DownloadUrlTorrentMagnet,
        directory: &str,
    ) -> Result<(), BitflowError> {
        // fetch .torrent file
        let output = Command::new("aria2c")
            .arg("-d")
            .arg(directory)
            .arg("--bt-save-metadata")
            .arg("--bt-metadata-only")
            .arg(&dl.magnet)
            .output()?;
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = output.split('\n').collect();
        let lines: Vec<&str> = lines
            .iter()
            .filter(|line| line.contains("Saved metadata as "))
            .copied()
            .collect();
        if lines.len() != 1 {
            return Err(BitflowError::Internal(
                "error fetching magnet metadata".to_string(),
            ));
        }
        let file_parts: Vec<&str> = lines[0].split("Saved metadata as ").collect();
        if file_parts.len() != 2 {
            return Err(BitflowError::Internal(
                "error parsing magnet metadata".to_string(),
            ));
        }
        let mut torrent_file_path = file_parts[1].to_string();
        torrent_file_path.pop(); // remove last trailing dot

        let output = Command::new("aria2c")
            .arg("-S")
            .arg(&torrent_file_path)
            .output()?;
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = output.split('\n').collect();
        let lines: Vec<&str> = lines
            .iter()
            .filter(|line| line.starts_with("Name: "))
            .copied()
            .collect();
        if lines.len() != 1 {
            return Err(BitflowError::Internal(
                "error fetching magnet metadata".to_string(),
            ));
        }

        let name: String = lines[0].chars().skip(6).collect();
        // TODO: send name to api
        if let Some(ref api_client) = self.api_client {
            let endpoint = format!(
                "{}/bitflow/v1/downloads/{}",
                self.api_endpoint.as_ref().unwrap_or(&"".to_string()),
                dl_id
            );
            api_client
                .put(&endpoint)
                .json(&models::UpdateNameBody { name })
                .send()?;
        }

        fs::remove_file(torrent_file_path)?;

        Command::new("aria2c")
            .arg("-d")
            .arg(directory)
            .arg("--seed-time")
            .arg("0")
            .arg(&dl.magnet)
            .output()?;

        return Ok(());
    }
}
