use anyhow::Result;
use generate::generate;

use crate::opt::{Command, DatabaseCommand, MigrateCommand};

mod database;
mod generate;
mod migrate;
mod opt;
mod shell;

pub use crate::opt::Opt;

pub async fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Shell(opt) => {
            shell::open_shell(opt).await?; 
        }

        Command::Migrate(migrate) => match migrate.command {
            MigrateCommand::Add {
                description,
                reversible,
            } => migrate::add(&migrate.source, &description, reversible).await?,
            MigrateCommand::Run {
                dry_run,
                ignore_missing,
                database_url,
            } => migrate::run(&migrate.source, &database_url, dry_run, ignore_missing).await?,
            MigrateCommand::Revert {
                dry_run,
                ignore_missing,
                database_url,
            } => migrate::revert(&migrate.source, &database_url, dry_run, ignore_missing).await?,
            MigrateCommand::Info { database_url } => {
                migrate::info(&migrate.source, &database_url).await?
            }
            MigrateCommand::BuildScript { force } => migrate::build_script(&migrate.source, force)?,
        },
        Command::Generate(gen_opt) => generate(gen_opt).await?,
        Command::Database(database) => match database.command {
            DatabaseCommand::Create { database_url } => database::create(&database_url).await?,
            DatabaseCommand::Drop { yes, database_url } => {
                database::drop(&database_url, !yes).await?
            }
            DatabaseCommand::Reset {
                yes,
                source,
                database_url,
            } => database::reset(&source, &database_url, !yes).await?,
            DatabaseCommand::Setup {
                source,
                database_url,
            } => database::setup(&source, &database_url).await?,
        },
    };

    Ok(())
}
