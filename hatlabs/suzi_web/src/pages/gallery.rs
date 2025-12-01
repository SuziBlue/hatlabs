use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    // State to store list of image filenames
    let images = use_state(Vec::<String>::new);
    // State for the selected image URL for modal
    let selected = use_state(|| None::<String>);

    {
        let images = images.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/images.json").send().await {
                    if let Ok(list) = resp.json::<Vec<String>>().await {
                        images.set(list);
                    }
                }
            });
            // Return a cleanup closure — no-op in this case
            || ()
        });
    }

    let on_image_click = {
        let selected = selected.clone();
        Callback::from(move |url: String| {
            selected.set(Some(url));
        })
    };

    let close_modal = {
        let selected = selected.clone();
        Callback::from(move |_| {
            selected.set(None);
        })
    };

    html! {
        <>
            <div class="gallery-grid">
                {
                    for (*images).iter().map(|name| {
                        let url = format!("images/{}", name);
                        let on_click = {
                            let url = url.clone();
                            let on_image_click = on_image_click.clone();
                            Callback::from(move |_| on_image_click.emit(url.clone()))
                        };
                        html! {
                            <div class="gallery-item">
                                <img src={url.clone()} alt={name.clone()} onclick={on_click} style="cursor: pointer;" />
                            </div>
                        }
                    })
                }
            </div>

            {
                if let Some(url) = &*selected {
                    html! {
                        <div class="modal-overlay" onclick={close_modal.clone()}>
                            <div class="modal-content" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                <button class="close-btn" onclick={close_modal.clone()}>{ "×" }</button>
                                <img src={url.clone()} alt="Full size image" />
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
