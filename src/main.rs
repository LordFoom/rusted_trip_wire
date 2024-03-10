use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args{
    ///This is the file to watch for any changes
    #[arg(short, long, value_name="FILE")]
    watch_file: String,
    ///This is the command that will be fired if the file changes
    #[arg(short, long, value_name="COMMAND")]
    trigger_command: String,
}
fn main() {
    println!("Hello, world!");
}
