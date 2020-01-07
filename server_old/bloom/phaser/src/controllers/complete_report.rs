use crate::{
    domain::{report, scan},
    models,
};
use actix::{Handler, Message};
use futures_fs::FsPool;
use kernel::{db::DbActor, KernelError};
use rusoto_s3::{PutObjectRequest, StreamingBody, S3};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
use zip;

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

    #[allow(clippy::unused_io_amount)]
    fn handle(&mut self, msg: CompleteReport, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{phaser_reports, phaser_scans};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            // unzip
            let report_zip = format!("{}/report.zip", &msg.report_dir);
            let file = fs::File::open(&report_zip)?;
            let archive = zip::ZipArchive::new(file)?;
            extarct_zip(msg.report_dir.clone(), archive)?;

            // send to S3
            for entry in WalkDir::new(&msg.report_dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let name = entry
                    .path()
                    .strip_prefix(Path::new(&msg.report_dir))
                    .expect("phaser: error unwraping report file path");
                let content_type = {
                    // read first 512 bytes to detect content type
                    let mut contents = [0u8; 512];
                    let mut file = fs::File::open(entry.path())?;
                    file.read(&mut contents)?;
                    mimesniff::detect_content_type(&contents)
                };

                let fspool = FsPool::default();
                let file_stream = fspool.read(
                    entry
                        .path()
                        .to_str()
                        .expect("error getting path")
                        .to_string(),
                    Default::default(),
                );

                // upload to s3
                // TODO: handle error
                let req = PutObjectRequest {
                    bucket: msg.s3_bucket.clone(),
                    key: format!(
                        "phaser/scans/{}/reports/{}/{}",
                        msg.scan_id,
                        msg.report_id,
                        name.display()
                    ),
                    body: Some(StreamingBody::new(file_stream)),
                    content_length: Some(entry.metadata()?.len() as i64),
                    content_type: Some(content_type.to_string()),
                    ..Default::default()
                };
                msg.s3_client
                    .put_object(req)
                    .sync()
                    .expect("pahser: Couldn't PUT object");
            }

            // parse report.json
            let report_path = format!("{}/report.json", &msg.report_dir);
            let report_contents = fs::read_to_string(&report_path)?;
            let parsed_report: models::report::Report = serde_json::from_str(&report_contents)?;
            let models::report::Report::V1(parsed_report) = parsed_report;
            let scan_id = parsed_report.scan_id;

            // generate report
            // TODO...
            let total_findings = parsed_report.targets[0]
                .findings
                .values()
                .fold(0u64, |acc, x| {
                    let y = match &x.result {
                        models::findings::ModuleResult::None
                        | models::findings::ModuleResult::Err(_) => 0,
                        _ => 1,
                    };
                    return acc + y;
                });

            // complete report
            // retrieve report
            let report_to_complete: report::Report = phaser_reports::dsl::phaser_reports
                .filter(phaser_reports::dsl::id.eq(parsed_report.id))
                .for_update()
                .first(&conn)?;

            let complete_cmd = report::Complete {
                findings: report::Finding::V1(parsed_report),
                total_findings,
            };
            let (completed_report, _) =
                eventsourcing::execute(&conn, report_to_complete, &complete_cmd)?;

            diesel::update(&completed_report)
                .set(&completed_report)
                .execute(&conn)?;

            // complete scan
            // retrieve Scan
            let scan_to_complete: scan::Scan = phaser_scans::dsl::phaser_scans
                .filter(phaser_scans::dsl::id.eq(scan_id))
                .for_update()
                .first(&conn)?;

            let complete_cmd = scan::Complete {
                findings: total_findings,
            };
            let (completed_scan, _) =
                eventsourcing::execute(&conn, scan_to_complete, &complete_cmd)?;

            diesel::update(&completed_scan)
                .set(&completed_scan)
                .execute(&conn)?;

            // remove files
            fs::remove_dir_all(&msg.report_dir)?;

            return Ok(());
        })?);
    }
}

fn extarct_zip<R: io::Read + io::Seek>(
    base_dir: String,
    mut archive: zip::read::ZipArchive<R>,
) -> Result<(), KernelError> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let filepath = file.sanitized_name();
        let outpath = format!(
            "{}/{}",
            &base_dir,
            filepath
                .to_str()
                .expect("phaser: error unwraping report file path")
        );

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
            if let Some(p) = filepath.parent() {
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
