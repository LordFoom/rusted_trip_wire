use clap::Parser;
use anyhow::Result;
use notify::{Config, PollWatcher, RecursiveMode, Watcher};
use std::path::Path;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args{
    ///This is the file to watch for any changes
    #[arg(short, long, value_name="FILE")]
    file_to_watch: String,
    ///This is the command that will be fired if the file changes
    #[arg(short, long, value_name="COMMAND")]
    command: String,
}
fn main()->Result<()> {
    //oh, there is a cool library we can use
    let mut watcher = notify::recommended_watcher(|res|{
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {}", e),
        }
    })?;
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    Ok(())
}
