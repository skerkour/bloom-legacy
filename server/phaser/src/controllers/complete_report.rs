use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
    events::EventMetadata,
};
use crate::domain;
use std::fs;
use std::io;
use std::io::prelude::*;
use zip;
use walkdir::WalkDir;
use rusoto_s3::{PutObjectRequest, S3};


#[derive(Clone)]
pub struct CompleteReport {
    pub report_dir: String,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub scan_id: uuid::Uuid,
    pub report_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteReport {
    type Result = Result<(), KernelError>;
}

impl Handler<CompleteReport> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: CompleteReport, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: None,
                request_id: Some(msg.request_id),
                session_id: None,
            };

            // unzip
            let report_zip = format!("{}/report.zip", &msg.report_dir);
            let file = fs::File::open(&report_zip)?;
            let archive = zip::ZipArchive::new(file)?;
            extarct_zip(archive)?;

            // send to S3
            for entry in WalkDir::new(&msg.report_dir).into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file()) {
                let mut contents: Vec<u8> = Vec::new();
                let file = fs::File::open(entry.path())?;
                let mut chunk = file.take(512);
                chunk.read(&mut contents)?;
                let content_type = mimesniff::detect_content_type(&contents);
                // upload to s3

                // TODO: handle error
                let req = PutObjectRequest {
                    bucket: msg.s3_bucket.clone(),
                    key: format!("phaser/{}/{}/{}", msg.scan_id, msg.report_id, entry.path().to_str().expect("phaser: error unwraping report file path")),
                    body: Some(contents.into()),
                    content_length: Some(entry.metadata()?.len() as i64),
                    content_type: Some(content_type.to_string()),
                    ..Default::default()
                };
                msg.s3_client.put_object(req).sync().expect("pahser: Couldn't PUT object");
            }
            // generate report
            // remove files

            return Ok(());
        })?);
    }
}

fn extarct_zip<R: io::Read + io::Seek>(mut archive: zip::read::ZipArchive<R>) -> Result<(), KernelError> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        // {
        //     let comment = file.comment();
        //     if !comment.is_empty() {
        //         // println!("File {} comment: {}", i, comment);
        //     }
        // }

        if (&*file.name()).ends_with('/') {
            // println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            fs::create_dir_all(&outpath)?;
        } else {
            // println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    return Ok(());
}
