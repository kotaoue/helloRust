use xilem::view::{button, flex, label};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

// アプリ全体の状態を1つの構造体で管理する
struct AppState {
    count: i32,
}

// 状態 → ビュー の純粋な関数。状態が変わるたびに呼ばれ、UIが再構築される
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    flex((
        label(format!("Count: {}", state.count)),
        button("Increment", |s: &mut AppState| s.count += 1),
        button("Decrement", |s: &mut AppState| s.count -= 1),
        button("Reset", |s: &mut AppState| s.count = 0),
    ))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState { count: 0 },
        app_logic,
        WindowOptions::new("Counter for Xilem"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
