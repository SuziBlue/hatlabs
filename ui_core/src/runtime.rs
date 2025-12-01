use core::time::Duration;

use crate::{components::Component, traits::{CanRender, Context}};







pub fn run_ui_loop<C, Ctx>(mut root: C, ctx: &mut Ctx)
where 
    C: Component,
    Ctx: Context + CanRender<C>,
{
    while !ctx.should_quit() {
        if let Err(e) = root.poll() {
            ctx.handle_error(e);
        }

        // Render updates
        if let Err(e) = ctx.render_setup() {
            ctx.handle_error(e);
        }

        if let Err(e) = ctx.render(&mut root) {
            ctx.handle_error(e);
        }

        // Sleep a bit to simulate frame/tick duration
        ctx.sleep(Duration::from_millis(16));
    }
}

