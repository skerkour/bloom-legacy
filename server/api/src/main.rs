// mod app;
use std::env;
use dotenv;

use sentry::integrations::panic::register_panic_handler;


use actix_web::{
    http::header, middleware::cors::Cors, middleware::Logger, web, App, HttpServer,
};
use rusoto_core::Region;
use rusoto_s3::S3Client;
use actix_http;
use actix::System;
use kernel::{
    log,
    log::macros::{slog_info, slog_o},
    config,
    db,
    api,
    accounts::api::v1 as accountsv1,
};
use std::str::FromStr;

use drive::api::v1 as drivev1;
use bitflow::api::v1 as bitflowv1;
use contacts::api::v1 as contactsv1;
use notes::api::v1 as notesv1;
use music::api::v1 as musicv1;
use gallery::api::v1 as galleryv1;


fn register_reactors() {
    // eventsourcing::subscribe::<_, account::Event, _>(Box::new(drive::reactors::AccountCreated{}));
    // eventsourcing::subscribe::<_, account::Event, _>(Box::new(bitflow::reactors::AccountCreated{}));
}


fn main() {
    dotenv::dotenv().ok();
    let _sentry_guard = sentry::init(env::var("SENTRY_URL").unwrap());
    env::set_var("RUST_BACKTRACE", "1");
    register_panic_handler();

    let (_log_guard, logger) = log::setup_slog();

    let sys = System::new("kernel");
    let cfg = config::init();
    let db_actor_addr = db::init(&cfg);
    let binding_addr = format!("0.0.0.0:{}", cfg.port());

    register_reactors();

    let region = Region::from_str(&cfg.aws_region()).expect("AWS region not valid");
    let api_state = api::State{
        db: db_actor_addr,
        config: cfg,
        s3_client: S3Client::new(region),
    };

    // TODO: logger middleware
    // TODO: sentry middleware
    HttpServer::new(move || {
        App::new()
        .data(api_state.clone())
        .wrap(
            Cors::new()
                .send_wildcard() // TODO...
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![header::ORIGIN, header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
                .max_age(3600)
        )
        .wrap(Logger::default())
        .wrap(api::middlewares::RequestIdMiddleware)
        .wrap(api::middlewares::AuthMiddleware)
        .service(web::resource("/").route(web::get().to(api::index)))

        // myaccount
        .service(web::resource("/myaccount/v1/registration/start")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::registration::start::post))
        )
        .service(web::resource("/myaccount/v1/registration/verify")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::registration::verify::post))
        )
        .service(web::resource("/myaccount/v1/registration/complete")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::registration::complete::post))
        )
        .service(web::resource("/myaccount/v1/registration/new-code")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::registration::new_code::post))
        )
        .service(web::resource("/myaccount/v1/sign-in")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::sign_in::post))
        )
        .service(web::resource("/myaccount/v1/sign-out")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::sign_out::post))
        )
        .service(web::resource("/myaccount/v1/recover")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::recover::post))
            .route(web::put().data(api::json_default_config()).to_async(accountsv1::recover::put))
        )
        .service(web::resource("/myaccount/v1/me")
            .route(web::get().data(api::json_default_config()).to(accountsv1::me::get))
            .route(web::put().data(api::json_default_config()).to_async(accountsv1::me::put))
        )
        .service(web::resource("/myaccount/v1/me/password")
            .route(web::put().data(api::json_default_config()).to_async(accountsv1::me::password::put))
        )
        .service(web::resource("/myaccount/v1/me/avatar")
            .route(web::put().data(api::json_default_config()).to_async(accountsv1::me::avatar::put))
        )
        .service(web::resource("/myaccount/v1/me/email")
            .route(web::put().data(api::json_default_config()).to_async(accountsv1::me::email::put))
        )
        .service(web::resource("/myaccount/v1/me/email/verify")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::me::email::verify::post))
        )
        .service(web::resource("/myaccount/v1/me/sessions")
            .route(web::get().to_async(accountsv1::me::sessions::get))
        )
        .service(web::resource("/myaccount/v1/me/sessions/{session_id}/revoke")
            .route(web::post().data(api::json_default_config()).to_async(accountsv1::me::sessions::revoke::post))
        )

        // drive
        .service(web::resource("/drive/v1/upload")
            .route(web::post().data(api::json_default_config()).to_async(drivev1::upload::post))
            .route(web::put().data(api::json_default_config()).to_async(drivev1::upload::put))
        )
        .service(web::resource("/drive/v1/me")
            .route(web::get().data(api::json_default_config()).to_async(drivev1::me::get))
        )
        .service(web::resource("/drive/v1/folders")
            .route(web::get().data(api::json_default_config()).to_async(drivev1::folders::get))
            .route(web::post().data(api::json_default_config()).to_async(drivev1::folders::post))
        )
        .service(web::resource("/drive/v1/files/{file_id}/url")
            .route(web::get().data(api::json_default_config()).to_async(drivev1::files::url::get))
        )
        .service(web::resource("/drive/v1/files/move")
            .route(web::post().data(api::json_default_config()).to_async(drivev1::files::move_::post))
        )
        .service(web::resource("/drive/v1/files/restore")
            .route(web::post().data(api::json_default_config()).to_async(drivev1::files::restore::post))
        )
        .service(web::resource("/drive/v1/files/delete")
            .route(web::post().data(api::json_default_config()).to_async(drivev1::files::delete::post))
        )
        .service(web::resource("/drive/v1/files/copy")
            .route(web::post().data(api::json_default_config()).to_async(drivev1::files::copy::post))
        )
        .service(web::resource("/drive/v1/trash")
            .route(web::get().data(api::json_default_config()).to_async(drivev1::trash::get))
            .route(web::post().data(api::json_default_config()).to_async(drivev1::trash::post))
        )

        // bitflow
        .service(web::resource("/bitflow/v1/downloads")
            .route(web::get().data(api::json_default_config()).to_async(bitflowv1::downloads::get))
            .route(web::post().data(api::json_default_config()).to_async(bitflowv1::downloads::post))
        )
        .service(web::resource("/bitflow/v1/downloads/remove")
            .route(web::post().data(api::json_default_config()).to_async(bitflowv1::downloads::remove::post))
        )
        .service(web::resource("/bitflow/v1/downloads/{download_id}")
            .route(web::get().data(api::json_default_config()).to_async(bitflowv1::downloads::download::get))
            .route(web::put().data(api::json_default_config()).to_async(bitflowv1::downloads::download::put))
        )
        .service(web::resource("/bitflow/v1/downloads/{download_id}/complete")
            .route(web::post().data(api::json_default_config()).to_async(bitflowv1::downloads::download::complete::post))
        )
        .service(web::resource("/bitflow/v1/downloads/{download_id}/fail")
            .route(web::post().data(api::json_default_config()).to_async(bitflowv1::downloads::download::fail::post))
        )
        .service(web::resource("/bitflow/v1/history")
            .route(web::get().data(api::json_default_config()).to_async(bitflowv1::history::get))
            .route(web::delete().data(api::json_default_config()).to_async(bitflowv1::history::delete))
        )
        .service(web::resource("/bitflow/v1/job")
            .route(web::get().data(api::json_default_config()).to_async(bitflowv1::job::get))
        )


        // contacts
        .service(web::resource("/contacts/v1/contacts")
            .route(web::get().data(api::json_default_config()).to_async(contactsv1::contacts::get))
            .route(web::post().data(api::json_default_config()).to_async(contactsv1::contacts::post))
        )
        .service(web::resource("/contacts/v1/contacts/{contact_id}")
            .route(web::get().data(api::json_default_config()).to_async(contactsv1::contacts::id::get))
            .route(web::put().data(api::json_default_config()).to_async(contactsv1::contacts::put))
            .route(web::delete().data(api::json_default_config()).to_async(contactsv1::contacts::delete))
        )

        // notes
        .service(web::resource("/notes/v1/notes")
            .route(web::get().data(api::json_default_config()).to_async(notesv1::notes::get))
            .route(web::post().data(api::json_default_config()).to_async(notesv1::notes::post))
        )
        .service(web::resource("/notes/v1/notes/{note_id}")
            .route(web::delete().data(api::json_default_config()).to_async(notesv1::notes::delete))
            .route(web::put().data(api::json_default_config()).to_async(notesv1::notes::put))
        )
        .service(web::resource("/notes/v1/notes/{note_id}/archive")
            .route(web::post().data(api::json_default_config()).to_async(notesv1::notes::archive::post))
        )
        .service(web::resource("/notes/v1/notes/{note_id}/unarchive")
            .route(web::post().data(api::json_default_config()).to_async(notesv1::notes::unarchive::post))
        )
        .service(web::resource("/notes/v1/notes/{note_id}/remove")
            .route(web::post().data(api::json_default_config()).to_async(notesv1::notes::remove::post))
        )
        .service(web::resource("/notes/v1/notes/{note_id}/restore")
            .route(web::post().data(api::json_default_config()).to_async(notesv1::notes::restore::post))
        )
        .service(web::resource("/notes/v1/archive")
            .route(web::get().data(api::json_default_config()).to_async(notesv1::archive::get))
        )
        .service(web::resource("/notes/v1/trash")
            .route(web::get().data(api::json_default_config()).to_async(notesv1::trash::get))
        )


        // music
        .service(web::resource("/music/v1/musics")
            .route(web::get().data(api::json_default_config()).to_async(musicv1::musics::get))
        )
        .service(web::resource("/music/v1/playlists")
            .route(web::get().data(api::json_default_config()).to_async(musicv1::playlists::get))
            .route(web::post().data(api::json_default_config()).to_async(musicv1::playlists::post))
        )
        .service(web::resource("/music/v1/playlists/{playlist_id}")
            .route(web::get().data(api::json_default_config()).to_async(musicv1::playlists::playlist::get))
            .route(web::delete().data(api::json_default_config()).to_async(musicv1::playlists::playlist::delete))
            .route(web::put().data(api::json_default_config()).to_async(musicv1::playlists::playlist::put))
        )
        .service(web::resource("/music/v1/playlists/{playlist_id}/add")
            .route(web::post().data(api::json_default_config()).to_async(musicv1::playlists::playlist::add::post))
        )
        .service(web::resource("/music/v1/playlists/{playlist_id}/remove")
            .route(web::post().data(api::json_default_config()).to_async(musicv1::playlists::playlist::remove::post))
        )

        // gallery
        .service(web::resource("/gallery/v1/media")
            .route(web::get().data(api::json_default_config()).to_async(galleryv1::media::get))
        )
        .service(web::resource("/gallery/v1/albums")
            .route(web::get().data(api::json_default_config()).to_async(galleryv1::albums::get))
            .route(web::post().data(api::json_default_config()).to_async(galleryv1::albums::post))
        )
        .service(web::resource("/gallery/v1/albums/{album_id}")
            .route(web::get().data(api::json_default_config()).to_async(galleryv1::albums::album::get))
            .route(web::delete().data(api::json_default_config()).to_async(galleryv1::albums::album::delete))
            .route(web::put().data(api::json_default_config()).to_async(galleryv1::albums::album::put))
        )
        .service(web::resource("/gallery/v1/albums/{album_id}/add")
            .route(web::post().data(api::json_default_config()).to_async(galleryv1::albums::album::add::post))
        )
        .service(web::resource("/gallery/v1/albums/{album_id}/remove")
            .route(web::post().data(api::json_default_config()).to_async(galleryv1::albums::album::remove::post))
        )
    })
    .backlog(8192)
    .keep_alive(actix_http::KeepAlive::Timeout(60))
    .shutdown_timeout(2)
    .workers(num_cpus::get())
    .bind(&binding_addr)
    .expect("error binding server")
    .start();

    slog_info!(logger, "server started"; slog_o!("address" => binding_addr));
    let _ = sys.run();
}
