use xilem::view::label;
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

fn app_logic(_data: &mut ()) -> impl WidgetView<()> {
    label("Hello, World!")
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple((), app_logic, WindowOptions::new("Hello World for Xilem"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
