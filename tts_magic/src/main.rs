use std::{error::Error, fs::File};

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tts_magic::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Error, 
                Config::default(), 
                TerminalMode::Mixed, 
                ColorChoice::Auto
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(".log")?
            ),
        ]
    )?;



    println!("Creating App.");
    let mut app = App::new()?;

    println!("Starting App.");
    let result = app.run().await;

    result
}
