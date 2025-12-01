use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
            <div class="page">
                { for props.children.iter() }
            </div>

            <footer class="site-footer">
                <div class="footer-container">
                    <div class="footer-about">
                        <h3>{ "About Suziblue" }</h3>
                        <p>
                            { "I'm Suziblue — writing about tech, code, and the occasional life lesson. 
                            Thanks for reading!" }
                        </p>
                    </div>

                    <div class="footer-contact">
                        <h3>{ "Contact" }</h3>
                        <p>{ "Email: " }<a href="mailto:suziblue@example.com">{ "suziblue@example.com" }</a></p>
                        <p>{ "GitHub: " }<a href="https://github.com/suziblue" target="_blank">{ "@suziblue" }</a></p>
                    </div>
                </div>

                <div class="footer-bottom">
                    <p>{ "© 2025 Suziblue. All rights reserved." }</p>
                </div>
            </footer>
        </>
    }
}
