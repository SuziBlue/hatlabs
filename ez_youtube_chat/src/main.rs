use ez_youtube_chat::youtube::tokens::get_youtube_token;
use ez_youtube_chat::youtube::api::get_livestreams;




fn main() {
    let token = get_youtube_token().await;

    let livestreams = get_livestreams(token);
    for livestream in livestreams {
        println!("{:?}", livestream);
    }
}
