use bloom_kernel::{
    accounts::controllers,
    // log::macros::slog_error,
    api,
    api::middlewares::{Auth, GetRequestAuth},
    config::Config,
};

use actix_web::{web, Error, HttpRequest, HttpResponse, Result as ActixResult};
use bloom_error::BloomError;
use bloom_messages::Message;
use futures::future::{ok, Future};
use futures_preview::{compat::Future01CompatExt, FutureExt, TryFutureExt}; // compat() converts futures::future::Future into a std::future::Future
use futures_timer::Delay;
use std::time::Duration;

pub fn config(_config: Config) -> impl Fn(&mut web::ServiceConfig) {
    return move |cfg| {
        cfg.service(
            web::resource("/")
                .route(web::get().to(get_index))
                .route(web::post().to_async(post_index)),
        );
    };
}

pub fn get_index() -> ActixResult<HttpResponse> {
    let res = api::response(bloom_messages::kernel::HelloWorld {
        hello: "world".to_string(),
    });
    return Ok(res);
}

pub fn post_index(
    message_wrapped: web::Json<Message>,
    state: web::Data<api::State>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let connection_info = req.connection_info();
    let remote = connection_info.remote();
    let auth = req.request_auth();
    // let logger = req.logger();

    let ip = match remote {
        Some(ref remote) => remote
            .split(':')
            .nth(0)
            .expect("Error accessing session ip")
            .to_string(),
        _ => "".to_string(),
    };

    return handle_message(state.into_inner(), auth, ip, message_wrapped.into_inner())
        .boxed()
        .compat()
        .then(move |res| match res {
            Ok(res) => ok(api::response(res)),
            Err(err) => {
                println!("ERRROR {}", &err);
                let err: bloom_messages::kernel::Error = err.into();
                return ok(api::response(err));
            }
        });
}

async fn handle_message(
    state: std::sync::Arc<api::State>,
    auth: Auth,
    ip: String,
    message: Message,
) -> Result<Message, BloomError> {
    match message {
        // Auth
        Message::AuthRegistrationStart(message) => {
            must_not_be_authenticated(&auth)?;

            let config = state.config.clone();

            let _ = Delay::new(Duration::from_millis(
                (400 + crypto42::rand::uniform(200)).into(), // 400-600 ms
            ))
            .await;

            state
                .db
                .send(controllers::StartRegistration { message, config })
                .compat()
                .await?
        }
        Message::AuthRegistrationVerify(message) => {
            must_not_be_authenticated(&auth)?;

            let _ = Delay::new(Duration::from_millis(
                (400 + crypto42::rand::uniform(200)).into(), // 400-600 ms
            ))
            .await;

            state
                .db
                .send(controllers::RegistrationVerify { message })
                .compat()
                .await?
        }
        Message::AuthRegistrationComplete(message) => {
            must_not_be_authenticated(&auth)?;

            let config = state.config.clone();

            let _ = Delay::new(Duration::from_millis(
                (400 + crypto42::rand::uniform(200)).into(), // 400-600 ms
            ))
            .await;

            state
                .db
                .send(controllers::CompleteRegistration {
                    message,
                    ip,
                    user_agent: "".to_string(), // TODO
                    config,
                })
                .compat()
                .await?
        }
        Message::AuthRegistrationNewCode(message) => {
            must_not_be_authenticated(&auth)?;

            let config = state.config.clone();

            let _ = Delay::new(Duration::from_millis(
                (400 + crypto42::rand::uniform(200)).into(), // 400-600 ms
            ))
            .await;

            state
                .db
                .send(controllers::RegistrationSendNewCode { message, config })
                .compat()
                .await?
        }
        Message::AuthSignIn(message) => {
            must_not_be_authenticated(&auth)?;

            let _ = Delay::new(Duration::from_millis(
                (400 + crypto42::rand::uniform(200)).into(), // 400-600 ms
            ))
            .await;

            state
                .db
                .send(controllers::SignIn {
                    message,
                    ip,
                    user_agent: "".to_string(), // TODO
                })
                .compat()
                .await?
        }
        Message::AuthSignOut(_) => {
            authentication_required(&auth)?;

            state
                .db
                .send(controllers::SignOut {
                    actor: auth.account.expect("error getting account from auth"),
                    session: auth.session.expect("error getting session from auth"),
                })
                .compat()
                .await?
        }
        Message::AuthRevokeSession(message) => {
            authentication_required(&auth)?;

            state
                .db
                .send(controllers::RevokeSession {
                    message,
                    actor: auth.account.expect("error getting account from auth"),
                    current_session_id: auth.session.expect("error getting session from auth").id,
                })
                .compat()
                .await?
        }
        _ => Err(BloomError::Validation("message is not valid".to_string())),
    }
}

fn must_not_be_authenticated(auth: &Auth) -> Result<(), BloomError> {
    if auth.session.is_some() || auth.account.is_some() || auth.service.is_some() {
        return Err(BloomError::Forbidden(
            "Must not be authenticated".to_string(),
        ));
    } else {
        return Ok(());
    }
}

fn authentication_required(auth: &Auth) -> Result<(), BloomError> {
    if auth.session.is_none() || auth.account.is_none() {
        return Err(BloomError::Forbidden("Authentication required".to_string()));
    } else {
        return Ok(());
    }
}

// pub fn config(config: Config) -> impl Fn(&mut web::ServiceConfig) {
//     return move |cfg| {
// cfg.service(web::resource("/").route(web::get().to(api::index)))
//             .service(
//                 web::scope("/myaccount")
//                     .route(
//                         "/v1/registration/start",
//                         web::post().to_async(myaccountv1::registration::start::post),
//                     )
//                     .route(
//                         "/v1/registration/verify",
//                         web::post().to_async(myaccountv1::registration::verify::post),
//                     )
//                     .route(
//                         "/v1/registration/complete",
//                         web::post().to_async(myaccountv1::registration::complete::post),
//                     )
//                     .route(
//                         "/v1/registration/new-code",
//                         web::post().to_async(myaccountv1::registration::new_code::post),
//                     )
//                     .route(
//                         "/v1/sign-in",
//                         web::post().to_async(myaccountv1::sign_in::post),
//                     )
//                     .route(
//                         "/v1/sign-out",
//                         web::post().to_async(myaccountv1::sign_out::post),
//                     )
//                     .service(
//                         web::resource("/v1/recover")
//                             .route(web::post().to_async(myaccountv1::recover::post))
//                             .route(web::put().to_async(myaccountv1::recover::put)),
//                     )
//                     .service(
//                         web::resource("/v1/me")
//                             .route(web::get().to(myaccountv1::me::get))
//                             .route(web::put().to_async(myaccountv1::me::put)),
//                     )
//                     .route(
//                         "/v1/me/password",
//                         web::put().to_async(myaccountv1::me::password::put),
//                     )
//                     .route(
//                         "/v1/me/avatar",
//                         web::put().to_async(myaccountv1::me::avatar::put),
//                     )
//                     .route(
//                         "/v1/me/email",
//                         web::put().to_async(myaccountv1::me::email::put),
//                     )
//                     .route(
//                         "/v1/me/email/verify",
//                         web::post().to_async(myaccountv1::me::email::verify::post),
//                     )
//                     .route(
//                         "/v1/me/sessions",
//                         web::get().to_async(myaccountv1::me::sessions::get),
//                     )
//                     .route(
//                         "/v1/me/sessions/{session_id}/revoke",
//                         web::post().to_async(myaccountv1::me::sessions::revoke::post),
//                     )
//                     .route(
//                         "/v1/users/{username}",
//                         web::get().to_async(myaccountv1::users::user::get),
//                     ),
//             )
//             // drive
//             .service(if config.disabled.drive {
//                 web::scope("/drive")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/drive")
//                     .route("/v1/me", web::get().to_async(drivev1::me::get))
//                     .route("/v1/uploads", web::post().to_async(drivev1::uploads::post))
//                     .route(
//                         "/v1/uploads/{upload_id}",
//                         web::put().to_async(drivev1::uploads::upload::put),
//                     )
//                     .service(
//                         web::resource("/v1/folders")
//                             .route(web::get().to_async(drivev1::folders::get))
//                             .route(web::post().to_async(drivev1::folders::post)),
//                     )
//                     .route(
//                         "/v1/files/{file_id}/url",
//                         web::get().to_async(drivev1::files::url::get),
//                     )
//                     .route(
//                         "/v1/files/{file_id}",
//                         web::put().to_async(drivev1::files::file::put),
//                     )
//                     .route(
//                         "/v1/files/move",
//                         web::post().to_async(drivev1::files::move_::post),
//                     )
//                     .route(
//                         "/v1/files/restore",
//                         web::post().to_async(drivev1::files::restore::post),
//                     )
//                     .route(
//                         "/v1/files/delete",
//                         web::post().to_async(drivev1::files::delete::post),
//                     )
//                     .route(
//                         "/v1/files/copy",
//                         web::post().to_async(drivev1::files::copy::post),
//                     )
//                     .service(
//                         web::resource("/v1/trash")
//                             .route(web::get().to_async(drivev1::trash::get))
//                             .route(web::post().to_async(drivev1::trash::post)),
//                     )
//             })
//             // bitflow
//             .service(if config.disabled.bitflow {
//                 web::scope("/bitflow")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/bitflow")
//                     .service(
//                         web::resource("/v1/downloads")
//                             .route(web::get().to_async(bitflowv1::downloads::get))
//                             .route(web::post().to_async(bitflowv1::downloads::post)),
//                     )
//                     .route(
//                         "/v1/downloads/remove",
//                         web::post().to_async(bitflowv1::downloads::remove::post),
//                     )
//                     .service(
//                         web::resource("/v1/downloads/{download_id}")
//                             .route(web::get().to_async(bitflowv1::downloads::download::get))
//                             .route(web::put().to_async(bitflowv1::downloads::download::put)),
//                     )
//                     .route(
//                         "/v1/downloads/{download_id}/complete",
//                         web::post().to_async(bitflowv1::downloads::download::complete::post),
//                     )
//                     .route(
//                         "/v1/downloads/{download_id}/fail",
//                         web::post().to_async(bitflowv1::downloads::download::fail::post),
//                     )
//                     .service(
//                         web::resource("/v1/history")
//                             .route(web::get().to_async(bitflowv1::history::get))
//                             .route(web::delete().to_async(bitflowv1::history::delete)),
//                     )
//                     .route("/v1/job", web::get().to_async(bitflowv1::job::get))
//             })
//             // contacts
//             .service(if config.disabled.contacts {
//                 web::scope("/contacts")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/contacts")
//                     .service(
//                         web::resource("/v1/contacts")
//                             .route(web::get().to_async(contactsv1::contacts::get))
//                             .route(web::post().to_async(contactsv1::contacts::post)),
//                     )
//                     .service(
//                         web::resource("/v1/contacts/{contact_id}")
//                             .route(web::get().to_async(contactsv1::contacts::id::get))
//                             .route(web::put().to_async(contactsv1::contacts::put))
//                             .route(web::delete().to_async(contactsv1::contacts::delete)),
//                     )
//             })
//             // notes
//             .service(if config.disabled.notes {
//                 web::scope("/notes")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/notes")
//                     .service(
//                         web::resource("/v1/notes")
//                             .route(web::get().to_async(notesv1::notes::get))
//                             .route(web::post().to_async(notesv1::notes::post)),
//                     )
//                     .service(
//                         web::resource("/v1/notes/{note_id}")
//                             .route(web::delete().to_async(notesv1::notes::delete))
//                             .route(web::put().to_async(notesv1::notes::put)),
//                     )
//                     .route(
//                         "/v1/notes/{note_id}/archive",
//                         web::post().to_async(notesv1::notes::archive::post),
//                     )
//                     .route(
//                         "/v1/notes/{note_id}/unarchive",
//                         web::post().to_async(notesv1::notes::unarchive::post),
//                     )
//                     .route(
//                         "/v1/notes/{note_id}/remove",
//                         web::post().to_async(notesv1::notes::remove::post),
//                     )
//                     .route(
//                         "/v1/notes/{note_id}/restore",
//                         web::post().to_async(notesv1::notes::restore::post),
//                     )
//                     .route("/v1/archive", web::get().to_async(notesv1::archive::get))
//                     .route("/v1/trash", web::get().to_async(notesv1::trash::get))
//             })
//             // music
//             .service(if config.disabled.music {
//                 web::scope("/music")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/music")
//                     .route("/v1/musics", web::get().to_async(musicv1::musics::get))
//                     .service(
//                         web::resource("/v1/playlists")
//                             .route(web::get().to_async(musicv1::playlists::get))
//                             .route(web::post().to_async(musicv1::playlists::post)),
//                     )
//                     .service(
//                         web::resource("/v1/playlists/{playlist_id}")
//                             .route(web::get().to_async(musicv1::playlists::playlist::get))
//                             .route(web::delete().to_async(musicv1::playlists::playlist::delete))
//                             .route(web::put().to_async(musicv1::playlists::playlist::put)),
//                     )
//                     .route(
//                         "/v1/playlists/{playlist_id}/add",
//                         web::post().to_async(musicv1::playlists::playlist::add::post),
//                     )
//                     .route(
//                         "/v1/playlists/{playlist_id}/remove",
//                         web::post().to_async(musicv1::playlists::playlist::remove::post),
//                     )
//             })
//             // gallery
//             .service(if config.disabled.gallery {
//                 web::scope("/gallery")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/gallery")
//                     .route("/v1/media", web::get().to_async(galleryv1::media::get))
//                     .service(
//                         web::resource("/v1/albums")
//                             .route(web::get().to_async(galleryv1::albums::get))
//                             .route(web::post().to_async(galleryv1::albums::post)),
//                     )
//                     .service(
//                         web::resource("/v1/albums/{album_id}")
//                             .route(web::get().to_async(galleryv1::albums::album::get))
//                             .route(web::delete().to_async(galleryv1::albums::album::delete))
//                             .route(web::put().to_async(galleryv1::albums::album::put)),
//                     )
//                     .route(
//                         "/v1/albums/{album_id}/add",
//                         web::post().to_async(galleryv1::albums::album::add::post),
//                     )
//                     .route(
//                         "/v1/albums/{album_id}/remove",
//                         web::post().to_async(galleryv1::albums::album::remove::post),
//                     )
//             })
//             // phaser
//             .service(if config.disabled.phaser {
//                 web::scope("/phaser")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/phaser")
//                     .route("/v1/job", web::get().to_async(phaserv1::job::get))
//                     .service(
//                         web::resource("/v1/scans")
//                             .route(web::get().to_async(phaserv1::scans::get))
//                             .route(web::post().to_async(phaserv1::scans::post)),
//                     )
//                     .route(
//                         "/v1/scans/{scan_id}",
//                         web::delete().to_async(phaserv1::scans::scan::delete),
//                     )
//                     .route(
//                         "/v1/scans/{scan_id}/queue",
//                         web::post().to_async(phaserv1::scans::scan::queue::post),
//                     )
//                     .route(
//                         "/v1/scans/{scan_id}/cancel",
//                         web::post().to_async(phaserv1::scans::scan::cancel::post),
//                     )
//                     .route(
//                         "/v1/scans/{scan_id}/reports",
//                         web::get().to_async(phaserv1::scans::scan::reports::get),
//                     )
//                     .route(
//                         "/v1/scans/{scan_id}/reports/{report_id}/complete",
//                         web::post()
//                             .to_async(phaserv1::scans::scan::reports::report::complete::post),
//                     )
//             })
//             // calendar
//             .service(if config.disabled.calendar {
//                 web::scope("/calendar")
//                     .service(web::resource("*").route(web::route().to(kernel::api::route_disabled)))
//             } else {
//                 web::scope("/calendar")
//                     .service(
//                         web::resource("/v1/events")
//                             .route(web::get().to_async(calendarv1::events::get))
//                             .route(web::post().to_async(calendarv1::events::post)),
//                     )
//                     .service(
//                         web::resource("/v1/events/{event_id}")
//                             .route(web::delete().to_async(calendarv1::events::event::delete))
//                             .route(web::put().to_async(calendarv1::events::event::put)),
//                     )
//             })
//             // admin
//             .service(
//                 web::scope("/admin")
//                     .route(
//                         "/v1/accounts/{account_id}/disable",
//                         web::post().to_async(adminv1::accounts::account::disable::post),
//                     )
//                     .route(
//                         "/v1/accounts/{account_id}/enable",
//                         web::post().to_async(adminv1::accounts::account::enable::post),
//                     )
//                     .route("/v1/accounts", web::get().to_async(adminv1::accounts::get))
//                     .service(
//                         web::resource("/v1/accounts/{account_id}")
//                             .route(web::get().to_async(adminv1::accounts::account::get))
//                             .route(web::delete().to_async(adminv1::accounts::account::delete)),
//                     ),
//             );
//     };
// }
