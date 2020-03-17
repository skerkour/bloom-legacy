use crate::{
    downloader,
    log::macros::*,
    worker::{config, models},
};
use futures_fs::FsPool;
use reqwest::header;
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};
use std::fs;
use std::str::FromStr;
use std::{thread, time};

#[derive(Clone)]
pub struct Worker {
    id: u64,
    config: config::Config,
    api_client: reqwest::Client,
    s3_client: S3Client,
    logger: slog::Logger,
    downloader: downloader::Downloader,
}

macro_rules! continue_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("An error: {}; continue.", e);
                continue;
            }
        }
    };
}

impl Worker {
    pub fn new(config: config::Config, id: u64) -> Worker {
        let mut headers = header::HeaderMap::new();
        let auth_header = format!("Secret bitflow:{}", &config.bloom_api_secret);
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header).unwrap(),
        );

        let logger = slog_scope::logger().new(slog_o!("worker" => id));

        let s3_client = S3Client::new_with(
            HttpClient::new().expect("failed to create request dispatcher"),
            EnvironmentProvider::default(),
            Region::from_str(&config.aws_region).unwrap(),
        );

        let api_client = reqwest::Client::builder()
            .gzip(true)
            .timeout(time::Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .expect("error building api client");
        return Worker {
            id,
            downloader: downloader::Downloader::new(
                Some(api_client.clone()),
                Some(config.bloom_api_url.clone()),
            ),
            config,
            api_client,
            s3_client,
            logger,
        };
    }

    pub fn run(&self) {
        let endpoint = format!("{}/bitflow/v1/job", self.config.bloom_api_url);
        slog_info!(self.logger, "worker started");

        loop {
            slog_info!(self.logger, "fetching job {}", &endpoint);
            let mut res = continue_fail!(self.api_client.get(&endpoint).send().map_err(|err| {
                thread::sleep(time::Duration::from_secs(10));
                return err;
            }));
            if res.status() == 200 {
                let res: models::ApiResponse = continue_fail!(res.json());
                let download = res.data.expect("error unwraping download job");
                let download_id = download.id;
                slog_info!(self.logger, "starting: {}", download.id);
                let result = self.downloader.download(download.into());
                match result {
                    Ok(downloaded) => {
                        // upload files + send complete
                        slog_info!(self.logger, "successfully downloaded"; "download" => download_id.to_hyphenated().to_string());
                        for file in &downloaded.files {
                            let fspool = FsPool::default();
                            let file_stream = fspool.read(file.path.clone(), Default::default());
                            let req = PutObjectRequest {
                                bucket: self.config.aws_s3_bucket.clone(),
                                key: format!("bitflow/{}/{}", download_id, file.bitflow_id),
                                body: Some(StreamingBody::new(file_stream)),
                                content_length: Some(file.size as i64),
                                content_type: Some(file.type_.clone()),
                                ..Default::default()
                            };
                            self.s3_client
                                .put_object(req)
                                .sync()
                                .expect("Couldn't PUT object");
                        }

                        slog_info!(self.logger, "files uploaded to S3"; "download" => download_id.to_hyphenated().to_string());

                        let endpoint = format!(
                            "{}/bitflow/v1/downloads/{}/complete",
                            self.config.bloom_api_url, download_id
                        );
                        continue_fail!(self
                            .api_client
                            .post(&endpoint)
                            .json(&models::CompleteBody {
                                files: downloaded.files
                            })
                            .send());

                        continue_fail!(fs::remove_dir_all(format!("downloads/{}", download_id)));
                    }
                    Err(err) => {
                        slog_warn!(self.logger, "download errored"; "download" => download_id.to_hyphenated().to_string());
                        let err = format!("{:?}", err);
                        let endpoint = format!(
                            "{}/bitflow/v1/downloads/{}/fail",
                            self.config.bloom_api_url, download_id
                        );
                        continue_fail!(self
                            .api_client
                            .post(&endpoint)
                            .json(&models::FailDownloadBody { error: err })
                            .send());
                    }
                }
            } else {
                slog_info!(self.logger, "no jobs, waiting 15 secs");
                thread::sleep(time::Duration::from_secs(15));
            }
        }
    }
}
