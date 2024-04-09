mod util;

use std::path::PathBuf;

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use notify::RecommendedWatcher;
use notify::{Config, RecursiveMode, Watcher};

use crate::util::confirm_backup_directory_if_provided;

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
    ///Line up a command to trigger, inbuilt variables are OLD_FILENAME and NEW_FILENAME
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
    if let Err(e) = watch(args.path_to_watch, args.backup_path, args.command) {
        println!("We had a terrible error! {}", e);
    }

    Ok(())
}

///Watch a directory and if a file in it is modified, and backup_path is provided,
///then make a copy of that file in the backup path
fn watch(
    path: String,
    maybe_backup_path: Option<String>,
    maybe_trigger_command: Option<String>,
) -> Result<()> {
    info!("Watching {}", path.to_string());
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    confirm_backup_directory_if_provided(&maybe_backup_path)?;

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
                        backup_file_if_required(&path, &maybe_backup_path)?;
                        run_command_if_required(&path, &maybe_backup_path, &maybe_trigger_command)?;
                    }
                }
                _ => info!("We do nothing"),
            },
            Err(err) => println!("ERRRROOORRRRR {err:?}"),
        }
    }

    Ok(())
}

pub fn run_command_if_required(
    old_path: &PathBuf,
    maybe_new_path: &Option<String>,
    maybe_command: &Option<String>,
) -> Result<()> {
    //short circuit if need be
    if &None == maybe_command {
        info!("No command supplied, no attempt to run a command will be made");
        return Ok(());
    }

    let cmd = maybe_command.as_ref().unwrap();

    Ok(())
}
///If a backup path has been provided, we copy the file, Optionally returning the new file nae
pub fn backup_file_if_required(
    path: &PathBuf,
    maybe_backup_path: &Option<String>,
) -> Result<Option<String>> {
    if let Some(ref backup_path) = maybe_backup_path {
        if let Some(os_filename) = path.file_name() {
            if let Some(filename) = os_filename.to_str() {
                //this is just the filename
                let bufn = construct_backup_file_name(filename);
                //now we add the backup directory
                let mut bufp = PathBuf::from(&backup_path);
                bufp.push(bufn);
                std::fs::copy(path.to_str().unwrap(), bufp.to_str().unwrap())?;
                let new_file_path = String::from(bufp.to_str().unwrap());
                let nfp_option = Some(new_file_path);
                return Ok(nfp_option);
            }
        }
    }
    Ok(None)
}

///Return bzsename.iso-8601-date
fn construct_backup_file_name(base_file_name: &str) -> String {
    let now = Local::now();
    let date_string = now.format("%Y-%m-%d_%H_%M_%S").to_string();
    let mut backup_file_name = base_file_name.to_string();
    backup_file_name.push('.');
    backup_file_name.push_str(&date_string);
    backup_file_name
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::construct_backup_file_name;

    #[test]
    pub fn test_construct_backup_file_name() {
        let new_file_name = construct_backup_file_name("test_file.txt");
        let test_regex = Regex::new(r".*\.\d\d\d\d-\d\d-\d\d_\d\d_\d\d_\d\d").unwrap();
        assert!(test_regex.is_match(&new_file_name));
    }

    #[test]
    pub fn test_backup_file_if_required() {
        //create a test file
        std::fs::File::create("this_is_a_test_file");
        let source_path = "./this_is_a_test_file.test";
        let destination_path = "./this_is_a_test_file.test.copy";
    }
}
