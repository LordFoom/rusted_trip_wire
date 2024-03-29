use clap::Parser;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use notify::RecommendedWatcher;
use notify::{Config, RecursiveMode, Watcher};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "Watch a folder and copy new and changed files to a backup directory"
)]
struct Args {
    ///This is the file to watch for any changes
    #[arg(short, long, value_name = "DIR_TO_WATCH")]
    path_to_watch: String,
    ///If provided, will backup files when created and when modified to this directory,
    ///will not delete
    #[arg(short, long, value_name = "BACKUP_DIR")]
    backup_path: Option<String>,
    ///This is the command that will be fired for creation and change
    #[arg(short, long, value_name = "COMMAND")]
    command: Option<String>,
    ///Output info while we run
    #[arg(short, long)]
    verbose: bool,
}

fn init_logging(verbose: bool) -> anyhow::Result<()> {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Error
    };

    let stdout = ConsoleAppender::builder().build();
    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))?;
    //am i going to use this?
    let _handle = log4rs::init_config(config)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_logging(args.verbose)?;
    //oh, there is a cool library we can use
    // let mut watcher = notify::recommended_watcher(|res| match res {
    //     Ok(event) => println!("event: {:?}", event),
    //     Err(e) => println!("watch error: {}", e),
    // })?;
    // watcher.watch(Path::new(&args.path_to_watch), RecursiveMode::Recursive)?;
    if let Err(e) = watch(args.path_to_watch, args.backup_path) {
        println!("We had a terrible error! {}", e);
    }

    Ok(())
}

///Watch a directory and if a file in it is modified, and backup_path is provided,
///then make a copy of that file in the backup path
fn watch(path: String, maybe_backup_path: Option<String>) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                    for path in event.paths {
                        info!(
                            "We received {:?}, for path: {}",
                            event.kind,
                            path.to_str().unwrap()
                        );
                    }
                    if let Some(ref backup_path) = maybe_backup_path {
                        //create a new filename based on time
                    }
                }
                _ => info!("We do nothing"),
            },
            Err(err) => println!("ERRRROOORRRRR {err:?}"),
        }
    }

    Ok(())
}
