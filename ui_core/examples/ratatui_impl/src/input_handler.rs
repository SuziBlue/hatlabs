use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::{Duration, SystemTime, UNIX_EPOCH}};

use ratatui::crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use ui_core::{comms::Sender, components::Component, inputs::{InputAction, InputCode, InputDevice, UserInputEvent, Vec2}};

use crate::channels::StdSender;


pub struct StdInputHandler {
    sender: StdSender<UserInputEvent<u16, i16>>,
    quit_flag: Arc<AtomicBool>,
}

impl StdInputHandler {
    pub fn new(sender: StdSender<UserInputEvent<u16, i16>>, quit_flag: Arc<AtomicBool>) -> Self {
        Self {
            sender,
            quit_flag,
        }
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
                        x: mouse_event.column,
                        y: mouse_event.row,
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
                        MouseEventKind::ScrollUp => InputCode::Scroll(Vec2 { x: 0, y: 1 }),
                        MouseEventKind::ScrollDown => InputCode::Scroll(Vec2 { x: 0, y: -1 }),
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
