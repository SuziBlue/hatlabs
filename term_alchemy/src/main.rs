use dotenv;
use std::error::Error;
use term_alchemy::app::{App, BasicEventStream, RenderArea, RunningState};
use term_alchemy::run;
use term_alchemy::widget::Widget;
use term_alchemy::youtube::api::get_livestreams;
use term_alchemy::youtube::get_youtube_token;
// fn main() -> Result<(), Error> {
//     // Load .env file
//     dotenv::dotenv().ok();
//
//     // let terminal = ratatui::init();
//     // let result = run(terminal);
//     // ratatui::restore();
//     // result
//
//     let token = get_youtube_token();
//     let livestreams = get_livestreams(token)
//         .unwrap()
//         .into_iter()
//         .filter(|livestream| livestream.status.lifeCycleStatus == "ready")
//         .collect::<Vec<_>>();
//     for livestream in livestreams {
//         println!("{:?}", livestream);
//     }
//
//     Ok(())
// }
//
//
fn main() -> Result<(), Box<dyn Error>> {
    let area1 = RenderArea {};
    let area2 = RenderArea {};

    let event_stream = BasicEventStream {};

    let widget1 = Widget::new(|| RunningState {}, &event_stream, &area1);
    let widget2 = Widget::new(|| RunningState {}, &event_stream, &area2);

    let app = App::new(vec![widget1, widget2]);

    app.run()
}
