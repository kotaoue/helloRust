use xilem::view::{flex_col, label, text_button};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

// アプリ全体の状態を1つの構造体で管理する
struct AppState {
    count: i32,
}

// 状態 → ビュー の純粋な関数。状態が変わるたびに呼ばれ、UIが再構築される
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    flex_col((
        label(format!("Count: {}", state.count)),
        text_button("Increment", |s: &mut AppState| s.count += 1),
        text_button("Decrement", |s: &mut AppState| s.count -= 1),
        text_button("Reset", |s: &mut AppState| s.count = 0),
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
