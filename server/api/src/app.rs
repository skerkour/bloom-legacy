use actix_web::{web, Result as ActixResult};
use actix_files;


use kernel::{
    myaccount::api::v1 as myaccountv1,
    api,
};
use drive::api::v1 as drivev1;
use bitflow::api::v1 as bitflowv1;
use contacts::api::v1 as contactsv1;
use notes::api::v1 as notesv1;
use music::api::v1 as musicv1;
use gallery::api::v1 as galleryv1;
use phaser::api::v1 as phaserv1;


// 404
pub fn p404() -> ActixResult<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("public/index.html")?.set_status_code(http::StatusCode::NOT_FOUND))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        .route("", web::get().to(api::index))
        .default_service(web::route().to(api::route_404))

         // myaccount
        .route("/myaccount/v1/registration/start", web::post().to_async(myaccountv1::registration::start::post))
        .route("/myaccount/v1/registration/verify", web::post().to_async(myaccountv1::registration::verify::post))
        .route("/myaccount/v1/registration/complete", web::post().to_async(myaccountv1::registration::complete::post))
        .route("/myaccount/v1/registration/new-code", web::post().to_async(myaccountv1::registration::new_code::post))
        .route("/myaccount/v1/sign-in", web::post().to_async(myaccountv1::sign_in::post))
        .route("/myaccount/v1/sign-out", web::post().to_async(myaccountv1::sign_out::post))
        .service(web::resource("/myaccount/v1/recover")
            .route(web::post().to_async(myaccountv1::recover::post))
            .route(web::put().to_async(myaccountv1::recover::put))
        )
        .service(web::resource("/myaccount/v1/me")
            .route(web::get().to(myaccountv1::me::get))
            .route(web::put().to_async(myaccountv1::me::put))
        )
        .route("/myaccount/v1/me/password", web::put().to_async(myaccountv1::me::password::put))
        .route("/myaccount/v1/me/avatar", web::put().to_async(myaccountv1::me::avatar::put))
        .route("/myaccount/v1/me/email", web::put().to_async(myaccountv1::me::email::put))
        .route("/myaccount/v1/me/email/verify", web::post().to_async(myaccountv1::me::email::verify::post))
        .route("/myaccount/v1/me/sessions", web::get().to_async(myaccountv1::me::sessions::get))
        .route("/myaccount/v1/me/sessions/{session_id}/revoke", web::post().to_async(myaccountv1::me::sessions::revoke::post))

        // drive
        .service(web::resource("/drive/v1/upload")
            .route(web::post().to_async(drivev1::upload::post))
            .route(web::put().to_async(drivev1::upload::put))
        )
        .route("/drive/v1/me", web::get().to_async(drivev1::me::get))
        .service(web::resource("/drive/v1/folders")
            .route(web::get().to_async(drivev1::folders::get))
            .route(web::post().to_async(drivev1::folders::post))
        )
        .route("/drive/v1/files/{file_id}/url", web::get().to_async(drivev1::files::url::get))
        .route("/drive/v1/files/move", web::post().to_async(drivev1::files::move_::post))
        .route("/drive/v1/files/restore", web::post().to_async(drivev1::files::restore::post))
        .route("/drive/v1/files/delete", web::post().to_async(drivev1::files::delete::post))
        .route("/drive/v1/files/copy", web::post().to_async(drivev1::files::copy::post))
        .service(web::resource("/drive/v1/trash")
            .route(web::get().to_async(drivev1::trash::get))
            .route(web::post().to_async(drivev1::trash::post))
        )

        // bitflow
        .service(web::resource("/bitflow/v1/downloads")
            .route(web::get().to_async(bitflowv1::downloads::get))
            .route(web::post().to_async(bitflowv1::downloads::post))
        )
        .route("/bitflow/v1/downloads/remove", web::post().to_async(bitflowv1::downloads::remove::post))
        .service(web::resource("/bitflow/v1/downloads/{download_id}")
            .route(web::get().to_async(bitflowv1::downloads::download::get))
            .route(web::put().to_async(bitflowv1::downloads::download::put))
        )
        .route("/bitflow/v1/downloads/{download_id}/complete", web::post().to_async(bitflowv1::downloads::download::complete::post))
        .route("/bitflow/v1/downloads/{download_id}/fail", web::post().to_async(bitflowv1::downloads::download::fail::post))
        .service(web::resource("/bitflow/v1/history")
            .route(web::get().to_async(bitflowv1::history::get))
            .route(web::delete().to_async(bitflowv1::history::delete))
        )
        .route("/bitflow/v1/job", web::get().to_async(bitflowv1::job::get))


        // contacts
        .service(web::resource("/contacts/v1/contacts")
            .route(web::get().to_async(contactsv1::contacts::get))
            .route(web::post().to_async(contactsv1::contacts::post))
        )
        .service(web::resource("/contacts/v1/contacts/{contact_id}")
            .route(web::get().to_async(contactsv1::contacts::id::get))
            .route(web::put().to_async(contactsv1::contacts::put))
            .route(web::delete().to_async(contactsv1::contacts::delete))
        )

        // notes
        .service(web::resource("/notes/v1/notes")
            .route(web::get().to_async(notesv1::notes::get))
            .route(web::post().to_async(notesv1::notes::post))
        )
        .service(web::resource("/notes/v1/notes/{note_id}")
            .route(web::delete().to_async(notesv1::notes::delete))
            .route(web::put().to_async(notesv1::notes::put))
        )
        .route("/notes/v1/notes/{note_id}/archive", web::post().to_async(notesv1::notes::archive::post))
        .route("/notes/v1/notes/{note_id}/unarchive", web::post().to_async(notesv1::notes::unarchive::post))
        .route("/notes/v1/notes/{note_id}/remove", web::post().to_async(notesv1::notes::remove::post))
        .route("/notes/v1/notes/{note_id}/restore", web::post().to_async(notesv1::notes::restore::post))
        .route("/notes/v1/archive", web::get().to_async(notesv1::archive::get))
        .route("/notes/v1/trash", web::get().to_async(notesv1::trash::get))


        // music
        .route("/music/v1/musics", web::get().to_async(musicv1::musics::get))
        .service(web::resource("/music/v1/playlists")
            .route(web::get().to_async(musicv1::playlists::get))
            .route(web::post().to_async(musicv1::playlists::post))
        )
        .service(web::resource("/music/v1/playlists/{playlist_id}")
            .route(web::get().to_async(musicv1::playlists::playlist::get))
            .route(web::delete().to_async(musicv1::playlists::playlist::delete))
            .route(web::put().to_async(musicv1::playlists::playlist::put))
        )
        .route("/music/v1/playlists/{playlist_id}/add", web::post().to_async(musicv1::playlists::playlist::add::post))
        .route("/music/v1/playlists/{playlist_id}/remove", web::post().to_async(musicv1::playlists::playlist::remove::post))

        // gallery
        .route("/gallery/v1/media", web::get().to_async(galleryv1::media::get))
        .service(web::resource("/gallery/v1/albums")
            .route(web::get().to_async(galleryv1::albums::get))
            .route(web::post().to_async(galleryv1::albums::post))
        )
        .service(web::resource("/gallery/v1/albums/{album_id}")
            .route(web::get().to_async(galleryv1::albums::album::get))
            .route(web::delete().to_async(galleryv1::albums::album::delete))
            .route(web::put().to_async(galleryv1::albums::album::put))
        )
        .route("/gallery/v1/albums/{album_id}/add", web::post().to_async(galleryv1::albums::album::add::post))
        .route("/gallery/v1/albums/{album_id}/remove", web::post().to_async(galleryv1::albums::album::remove::post))

        // phaser
        .route("/phaser/v1/job", web::get().to_async(phaserv1::job::get))
        .service(web::resource("/phaser/v1/scans")
            .route(web::get().to_async(phaserv1::scans::get))
            .route(web::post().to_async(phaserv1::scans::post))
        )
        .route("/phaser/v1/scans/{scan_id}", web::delete().to_async(phaserv1::scans::scan::delete))
        .route("/phaser/v1/scans/{scan_id}/queue", web::post().to_async(phaserv1::scans::scan::queue::post))
        .route("/phaser/v1/scans/{scan_id}/cancel", web::post().to_async(phaserv1::scans::scan::cancel::post))
        .route("/phaser/v1/scans/{scan_id}/reports", web::get().to_async(phaserv1::scans::scan::reports::get))
        .route("/phaser/v1/scans/{scan_id}/reports/{report_id}/complete", web::post().to_async(phaserv1::scans::scan::reports::report::complete::post))
    )
    .service(
        // serve webapp
        actix_files::Files::new("/", "public/")
        .index_file("index.html")
        .default_handler(web::route().to(p404))
    );
}
