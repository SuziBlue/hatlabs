use std::{error::Error, io::{stdout, Write}, marker::PhantomData, sync::mpsc::channel, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

use crossterm::{cursor::{self, MoveTo}, event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind}, execute, style::{self, Color, Print, ResetColor, SetForegroundColor, Stylize}, terminal::{disable_raw_mode, enable_raw_mode}, QueueableCommand};
use ui_core::{comms::{Receiver, Sender}, components::{Button, ButtonClick, Component, Cons, Label, Nil, Reactive, Region, Transform}, inputs::{InputAction, InputCode, InputDevice, UserInputEvent, Vec2}, runtime::run_ui_loop, CanRender, Context, RenderError};

use std::sync::mpsc;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone)]
pub struct StdSender<T>(pub mpsc::Sender<T>);

impl<T: Send + Sync + Clone> ui_core::comms::Sender<T> for StdSender<T> {
    fn try_send(&self, msg: T) -> Result<(), ui_core::comms::SendError> {
        self.0.send(msg).map_err(|_| ui_core::comms::SendError::Disconnected)
    }
}

#[derive(Debug)]
pub struct StdReceiver<T>(pub mpsc::Receiver<T>);

impl<T: Send + Sync + Clone> ui_core::comms::Receiver<T> for StdReceiver<T> {
    fn try_recv(&mut self) -> Result<T, ui_core::comms::RecvError> {
        self.0.try_recv().map_err(|err| match err {
            mpsc::TryRecvError::Empty => ui_core::comms::RecvError::Empty,
            mpsc::TryRecvError::Disconnected => ui_core::comms::RecvError::Disconnected,
        })
    }
}

pub struct StdContext {
    quit_flag: Arc<AtomicBool>,
}

impl StdContext {
    pub fn new(quit_flag: Arc<AtomicBool>) -> Self {
        Self { quit_flag }
    }
}

impl Context for StdContext {
    fn sleep(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
    fn handle_error(&self, err: impl Error) {
        eprintln!("Poll error: {:?}", err)
    }
    fn render_setup(&self) -> Result<(), ui_core::RenderError> {
        print!("\x1B[2J\x1B[H");
        std::io::stdout().flush().unwrap();

        Ok(())
    }
    fn should_quit(&self) -> bool {
        self.quit_flag.load(Ordering::Relaxed)
    }
}

impl<H, T> CanRender<Cons<H, T>> for StdContext 
where 
    H: Component,
    T: Component,
    StdContext: CanRender<H>,
    StdContext: CanRender<T>,
{
    fn render(
            &self, 
            component: &Cons<H, T>,
        ) -> Result<(), ui_core::RenderError> {
        component.head.render(self)?;
        component.tail.render(self)
    }
}

impl CanRender<Nil> for StdContext {
    fn render(
            &self, 
            component: &Nil,
        ) -> Result<(), ui_core::RenderError> {
        Ok(())
    }
}

impl<A, B, R, W, S, F> CanRender<Transform<A, B, R, W, S, F>> for StdContext
where
    R: Receiver<A>,
    W: Sender<B>,
    F: FnMut(&mut S, A) -> B,
    S: Clone + Default,
{
    fn render(
            &self, 
            component: &Transform<A, B, R, W, S, F>,
        ) -> Result<(), ui_core::RenderError> {
        Ok(())        
    }
}

impl CanRender<Label<String>> for StdContext {
    fn render(
            &self, 
            component: &Label<String>,
        ) -> Result<(), ui_core::RenderError> {
        println!("Label: {}", component.text);
        Ok(())
    }
}

impl<C, R> CanRender<Reactive<C, R>> for StdContext 
where
    C: Component,
    R: Receiver<C::State>,
    StdContext: CanRender<C>,
{
    fn render(
            &self, 
            component: &Reactive<C, R>,
        ) -> Result<(), ui_core::RenderError> {
        component.component.render(self)
    }
}

impl CanRender<StdInputHandler> for StdContext {
    fn render(
            &self, 
            component: &StdInputHandler,
        ) -> Result<(), ui_core::RenderError> {
        Ok(())
    }
}

struct StdInputHandler {
    sender: StdSender<UserInputEvent>,
    quit_flag: Arc<AtomicBool>,
}

impl StdInputHandler {
    fn new(sender: StdSender<UserInputEvent>, quit_flag: Arc<AtomicBool>) -> Self {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnableMouseCapture).unwrap();
        Self {
            sender,
            quit_flag,
        }
    }
}

impl Drop for StdInputHandler {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(stdout(), DisableMouseCapture).unwrap();
    }
}

impl Component for StdInputHandler {
    type State = ();

    fn poll(&mut self) -> Result<(), ui_core::comms::ChannelError> {
        fn map_mouse_button(button: MouseButton) -> u8 {
            match button {
                MouseButton::Left => 1,
                MouseButton::Right => 2,
                MouseButton::Middle => 3,
            }
        }
        if event::poll(Duration::from_millis(1)).unwrap() {
            let event = event::read().unwrap();
            let input_event = match event {
                Event::Key(key_event) => {
                    let code = match key_event.code {
                        KeyCode::Char('q') => {
                            self.quit_flag.store(true, Ordering::Relaxed);
                            return Ok(()); // Exit early
                        }
                        KeyCode::Char(c) => InputCode::Key {
                            button: c,
                            action: InputAction::Pressed, // You could refine this with key_event.kind
                        },
                        KeyCode::F(n) => InputCode::KeyCode {
                            button: n as u32,
                            action: InputAction::Pressed,
                        },
                        _ => return Ok(()), // crude fallback
                    };

                    UserInputEvent {
                        device: InputDevice::Keyboard,
                        code,
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    }
                }

                Event::Mouse(mouse_event) => {
                    let position = Vec2 {
                        x: mouse_event.column as f32,
                        y: mouse_event.row as f32,
                    };

                    let code = match mouse_event.kind {
                        MouseEventKind::Down(btn) => InputCode::MouseButton {
                            button: map_mouse_button(btn),
                            action: InputAction::Pressed,
                            position,
                        },
                        MouseEventKind::Up(btn) => InputCode::MouseButton {
                            button: map_mouse_button(btn),
                            action: InputAction::Released,
                            position,
                        },
                        MouseEventKind::Moved => InputCode::MousePosition(position),
                        MouseEventKind::ScrollUp => InputCode::Scroll(Vec2 { x: 0.0, y: 1.0 }),
                        MouseEventKind::ScrollDown => InputCode::Scroll(Vec2 { x: 0.0, y: -1.0 }),
                        _ => return Ok(()),
                    };

                    UserInputEvent {
                        device: InputDevice::Mouse,
                        code,
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    }
                }

                _ => return Ok(()), // Ignore resize or unsupported
            };

            return Ok(self.sender.try_send(input_event)?);
        }

        Ok(())
    }
}


struct StdRegion {
    top_left: Vec2,
    bottom_right: Vec2,
}

impl StdRegion {
    fn width(&self) -> f32 {
        self.bottom_right.x - self.top_left.x
    }
    fn height(&self) -> f32 {
        self.bottom_right.y - self.top_left.y
    }
    fn center(&self) -> Vec2 {
        Vec2 { 
            x: self.top_left.x + self.width() / 2.0, 
            y: self.top_left.y + self.height() / 2.0, 
        }
    }
}

impl Region for StdRegion {
    fn is_inside(&self, position: Vec2) -> bool {
        position.x >= self.top_left.x
            && position.x <= self.bottom_right.x
            && position.y >= self.top_left.y
            && position.y <= self.bottom_right.y
    }
}

impl<S, R, T> CanRender<Button<S, R, T, StdRegion>> for StdContext 
where 
    R: Receiver<UserInputEvent>,
    S: Sender<ButtonClick>,
    T: AsRef<str> + Default + Ord + Clone,
{
    fn render(
            &self, 
            component: &Button<S, R, T, StdRegion>,
        ) -> Result<(), ui_core::RenderError> {

        let Vec2 { x: x_min, y: y_min } = component.region.top_left;
        let Vec2 { x: x_max, y: y_max } = component.region.bottom_right;
        let label_text = component.label.text.as_ref(); // assuming Label<T> exposes text() -> &T

        let mut stdout = stdout();
        for y in y_min as u16..y_max as u16 {
            for x in x_min as u16..x_max as u16 {
                if (y == y_min as u16 || y == y_max as u16 - 1) || (x == x_min as u16 || x == x_max as u16 - 1) {
                    // in this loop we are more efficient by not flushing the buffer.
                    stdout
                        .queue(cursor::MoveTo(x,y)).map_err(|_| RenderError)?
                        .queue(style::PrintStyledContent( "█".magenta())).map_err(|_| RenderError)?;
                }
            }
        }

        let Vec2 { x: center_x, y: center_y } = component.region.center();

        stdout
            .queue(MoveTo(center_x as u16, center_y as u16)).map_err(|_| RenderError)?
            .queue(SetForegroundColor(Color::White)).map_err(|_| RenderError)?
            .queue(Print(format!("[ {} ]", label_text))).map_err(|_| RenderError)?
            .queue(ResetColor).map_err(|_| RenderError)?
            .flush().map_err(|_| RenderError)?;

        Ok(())
    }
}

fn main() {

    let (click_tx, click_rx) = channel::<UserInputEvent>();
    let (label_tx, label_rx) = channel::<String>();
    let (button_tx, button_rx) = channel::<ButtonClick>();

    let quit_flag = Arc::new(AtomicBool::new(false));
    let quit_flag_input = quit_flag.clone();

    let input_handler = StdInputHandler::new(StdSender(click_tx), quit_flag_input);

    let click_counter = Transform::new(StdReceiver(button_rx), StdSender(label_tx), 0usize, |count, _click| {
        *count += 1;

        let msg = format!("Clicked {} times", *count);

        msg
    });

    let label = Label {
        text: "Initial".to_string(),
        style: None,
    };

    let reactive_label = Reactive {
        component: label,
        receiver: StdReceiver(label_rx),
    };

    let button = Button::new(
        StdSender(button_tx), 
        StdReceiver(click_rx), 
        Label { text: "Button".to_string(), style: None }, 
        StdRegion { 
            top_left: Vec2 { x: 10.0, y: 10.0 },
            bottom_right: Vec2 { x: 60.0, y: 16.0 },
        }
    );

    let root = Cons {
        head: input_handler,
        tail: Cons {
            head: click_counter,
            tail: Cons {
                head: reactive_label,
                tail: Cons {
                    head: button,
                    tail: Nil,
                },
            },
        },
    };

    let ctx = StdContext::new(quit_flag);

    run_ui_loop(root, &ctx)
}
