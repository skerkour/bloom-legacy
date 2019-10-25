use bloom_kernel::{
    config,
    db,
    log::macros::slog_info,
    // myaccount::domain::account,
};
use diesel_migrations::embed_migrations;
use std::error::Error;

// Embeds all migrations into the resulting binary file.
embed_migrations!();

pub fn run(cfg: config::Config, logger: slog::Logger) -> Result<(), Box<dyn Error>> {
    let pool = db::get_pool_db_conn(&cfg);
    let connection = pool.get()?;
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())?;

    slog_info!(logger, "Finished migrations");
    return Ok(());
}

pub fn revert(cfg: config::Config, logger: slog::Logger) -> Result<(), Box<dyn Error>> {
    let pool = db::get_pool_db_conn(&cfg);
    let connection = pool.get()?;
    diesel_migrations::revert_latest_migration(&connection)?;

    slog_info!(logger, "Finished reverting latest migration");
    return Ok(());
}
