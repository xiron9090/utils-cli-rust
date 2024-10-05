use std::env;
use utils_cli::Commands;

fn main() {
    //WARN demo 1
    let args: Vec<String> = env::args().collect();
    let command = Commands::new(&args).unwrap();
    command.execute().unwrap();
}
