mod parse_onnx;

use gloo::file::callbacks;
use gloo::file::File;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yew::Renderer;

#[function_component(App)]
fn app() -> Html {
    let file_content = use_state(|| None);
    let _reader = use_state(|| None); // Keep reader alive

    let on_file_change = {
        let file_content = file_content.clone();
        let _reader = _reader.clone();

        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event
                .target()
                .unwrap()
                .dyn_into()
                .unwrap();

            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file = File::from(file);
                    let file_name = file.name();

                    let fc = file_content.clone();
                    let reader = callbacks::read_as_text(&file, move |result| {
                        match result {
                            Ok(text) => {
                                fc.set(Some(format!("{}:\n{}", file_name, text)));
                            }
                            Err(err) => {
                                fc.set(Some(format!("Error reading file: {:?}", err)));
                            }
                        }
                    });

                    _reader.set(Some(reader)); // Keep alive
                }
            }
        })
    };

    html! {
        <div>
            <h1>{ "File Upload in Yew" }</h1>
            <input type="file" onchange={on_file_change} />
            {
                if let Some(content) = &*file_content {
                    html! {
                        <pre>{ content }</pre>
                    }
                } else {
                    html! { <p>{ "No file selected." }</p> }
                }
            }
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
