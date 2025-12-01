
use std::rc::Rc;
use std::cell::Cell;
use gloo::timers::callback::Timeout;
use gloo::console::error;
use std::time::Duration;
use wasm_bindgen::JsCast;

pub struct WasmContext {
    quit_flag: Rc<Cell<bool>>,
    document: Document,
    root: Element, // Root DOM node to render into
}

impl WasmContext {
    pub fn new() -> Result<Self, RenderError> {
        let window = web_sys::window().ok_or(RenderError)?;
        let document = window.document().ok_or(RenderError)?;
        let root = document
            .get_element_by_id("app")
            .ok_or(RenderError)?;

        Ok(Self {
            quit_flag: Rc::new(Cell::new(false)),
            document,
            root,
        })
    }

    pub fn request_quit(&self) {
        self.quit_flag.set(true);
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn root(&self) -> &Element {
        &self.root
    }
}

impl Context for WasmContext {
    fn sleep(&self, duration: Duration) {
        let _ = gloo::timers::callback::Timeout::new(duration.as_millis() as u32, || {}).forget();
    }

    fn handle_error(&self, err: impl Error) {
        error!(format!("Error: {}", err));
    }

    fn render_setup(&self) -> Result<(), RenderError> {
        // Clear all child nodes before re-rendering
        while let Some(child) = self.root.first_child() {
            self.root.remove_child(&child).map_err(|_| RenderError)?;
        }
        Ok(())
    }

    fn should_quit(&self) -> bool {
        self.quit_flag.get()
    }
}
