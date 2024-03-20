use clap::Parser;
use log::info;
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
    ///This is the command that will be fired if the file changes
    #[arg(short, long, value_name = "COMMAND")]
    command: Option<String>,
}

fn main() -> notify::Result<()> {
    let args = Args::parse();
    //oh, there is a cool library we can use
    // let mut watcher = notify::recommended_watcher(|res| match res {
    //     Ok(event) => println!("event: {:?}", event),
    //     Err(e) => println!("watch error: {}", e),
    // })?;
    // watcher.watch(Path::new(&args.path_to_watch), RecursiveMode::Recursive)?;
    if let Err(e) = watch(&args.path_to_watch) {
        println!("We had a terrible error! {}", e);
    }

    Ok(())
}

fn watch(path: &str) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Create(_) | notify::EventKind::Modify(_) => {
                    for path in event.paths {
                        println!(
                            "We received {:?}, for path: {}",
                            event.kind,
                            path.to_str().unwrap()
                        );
                    }
                }
                _ => info!("We do nothing"),
            },
            Err(err) => println!("ERRRROOORRRRR {err:?}"),
        }
    }

    Ok(())
}
