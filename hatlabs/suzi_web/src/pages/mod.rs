use yew::prelude::*;

pub mod layout;
pub mod gallery;
pub mod blog;
pub mod schedule;
pub mod blog_post_detail;

pub use layout::Layout;
pub use gallery::Gallery;
pub use blog::Blog;
pub use schedule::Schedule;
pub use blog_post_detail::BlogPostDetail;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1>{ "The Official Suziblue Homepage!" }</h1>
            <p>{ "Welcome to the world of midnight snacks and magical misfortune." }</p>
            <div style = "display: flex; flex-direction: column; align-items: center;">
                <img style="width: 200px" src="http://localhost:3030/blueberry.gif"/>
                <span style="font-size: 12pt;">{ "Art by: napixelz" }</span>
            </div>
        </>
    }
}


#[function_component(Lore)]
pub fn lore() -> Html {
    html! {
        <>
            <h2>{ "Suziblue's Lore" }</h2>
            <p>
                { "Suziblue was once a top alchemy student, until a tragic potion accident left her with the consistency of mashed jelly. " }
                { "Now she roams the digital realm, melting through keyboards and surviving off gamer snacks, all while enduring your questionable company. " }
                { "After the accident, Suziblue tried living a normal life, but people kept slipping on her in the hallways. " }
                { "So she retreated into the internet, learned arcane gaming, and was adopted by the world’s greatest magical hat, as her only friend. " }
                { "Together, we set out on a quest to master the art of streaming, armed with questionable magic, endless sarcasm, and a gift for technical difficulties. " }<br /><br />
                <em>{ "\"Every epic starts somewhere—I just wish ours started after a nap.\" " }</em><br /><br />
                <span style="display: block; text-align: right; padding-right: 10rem;">
                    { "- Suziblue" }
                </span>
            </p>
        </>
    }
}

#[function_component(FAQ)]
pub fn faq() -> Html {
    html! { 
        <>
            <h1>{ "FAQ Page" }</h1> 
            <p>
                { "Q: Does Suziblue melt in the rain?" }<br />
                { "A: \"Only a little, but honestly, it feels kind of nice—like a spa day for slimes, with 80% more existential dread.\"" }<br /><br />
                { "Q: Why is Suziblue always tired?" }<br />
                { "A: \"Because sleep is just a rumor in the wizarding world, and gaming until 4AM is my only real spell.\"" }<br /><br />
                { "Q: Can Suziblue eat normal food?" }<br />
                { "A: \"No, I cannot. Mostly energy drinks, sadness, and the slow march of time. Sometimes a healing potion smoothie if I'm feeling fancy.\"" }<br /><br />
            </p>
        </>
    }
}


#[function_component(Socials)]
pub fn socials() -> Html {
    html! {
        <>
            <h2>{ "Contact Suziblue" }</h2>
            <ul>
                <li>
                    <a href="https://youtube.com/@magicwizardhat" target="_blank" rel="noopener noreferrer">
                        { "Youtube" }
                    </a>
                </li>
                <li>
                    <a href="https://twitch.tv/magicwizardhat" target="_blank" rel="noopener noreferrer">
                        { "Twitch" }
                    </a>
                </li>
                <li>
                    <a href="https://x.com/MagicWizardHat" target="_blank" rel="noopener noreferrer">
                        { "X / Twitter" }
                    </a>
                </li>
                <li>
                    <a href="https://discord.gg/JUQPvBXrjv" target="_blank" rel="noopener noreferrer">
                        { "Discord" }
                    </a>
                </li>
            </ul>
        </>
    }
}


#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! { <h1>{ "404 - Page Not Found" }</h1> }
}
