use crate::app::AppActions;
use std::io;
use crossbeam_channel::{Sender, Receiver};

pub struct InputHandler {}


impl InputHandler {
    pub async fn run(user_input_tx: Sender<AppActions>) {
        let stdin = io::stdin();

        loop {
            print!("> ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let mut input = String::new();
            if stdin.read_line(&mut input).is_err() {
                eprintln!("Failed to read input");
                continue;
            }

            let mut input_tokens = input.trim().split_whitespace();
            if let Some(command) = input_tokens.next() {

                let args: Vec<String> = input_tokens.map(String::from).collect();

                let action = match command.to_lowercase().as_str() {
                    "devices" => AppActions::DisplayDevices,
                    "test" => AppActions::TestOutputDevice,
                    "quit" => AppActions::QuitApp,
                    "inputtest" => AppActions::TestInputDevice,
                    "startstream" => AppActions::StartStream,
                    "listlivestreams" => AppActions::ListLivestreams,
                    "testocr" => AppActions::TestOCR,
                    "help" => {
                        println!("Available commands are:");
                        println!("  devices");
                        println!("  test");
                        println!("  quit");
                        println!("  inputtest");
                        println!("  startstream");
                        println!("  listlivestreams");
                        println!("  testocr");
                        println!("  help");
                        continue;
                    }
                    _ => {
                        println!("Unknown command: {}", command);
                        continue;
                    }
                };

                if user_input_tx.send(action.clone()).is_err() {
                    println!("Main loop dropped. Exiting input handler.");
                    break;
                }
                if action == AppActions::QuitApp {
                    println!("Quit action sent. Exiting input handler.");
                    break;
                }
            } else {
                continue;
            }
        }
    }
}
