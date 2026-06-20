use xilem::view::{
    checkbox, flex_col, flex_row, indexed_stack, label, sized_box, text_button, text_input,
    virtual_scroll, CrossAxisAlignment, FlexExt as _,
};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Todo,
}

impl Page {
    fn index(self) -> usize {
        match self {
            Page::Home => 0,
            Page::Todo => 1,
        }
    }
}

#[derive(Clone)]
struct TodoItem {
    title: String,
    done: bool,
}

// アプリ全体の状態を1つの構造体で管理する
struct AppState {
    page: Page,
    count: i32,
    draft: String,
    todos: Vec<TodoItem>,
}

impl AppState {
    fn add_todo(&mut self) {
        let title = self.draft.trim();
        if title.is_empty() {
            return;
        }

        self.todos.push(TodoItem {
            title: title.to_owned(),
            done: false,
        });
        self.draft.clear();
    }
}

fn navigation(_state: &mut AppState) -> impl WidgetView<AppState> {
    flex_row((
        text_button("Home", |state: &mut AppState| {
            state.page = Page::Home;
        }),
        text_button("TODO", |state: &mut AppState| {
            state.page = Page::Todo;
        }),
    ))
}

fn home_page(state: &mut AppState) -> impl WidgetView<AppState> {
    flex_col((
        label("Counter demo"),
        label(format!("Count: {}", state.count)),
        flex_row((
            text_button("-1", |state: &mut AppState| state.count -= 1),
            text_button("+1", |state: &mut AppState| state.count += 1),
            text_button("Reset", |state: &mut AppState| state.count = 0),
        )),
    ))
}

fn todo_rows(state: &mut AppState) -> impl WidgetView<AppState> {
    virtual_scroll(0..state.todos.len() as i64, |state: &mut AppState, idx| {
        let idx = idx as usize;
        let todo = state.todos[idx].clone();

        flex_row((
            checkbox(todo.title, todo.done, move |state: &mut AppState, checked| {
                state.todos[idx].done = checked;
            }),
            text_button("Delete", move |state: &mut AppState| {
                state.todos.remove(idx);
            }),
        ))
    })
}

fn todo_page(state: &mut AppState) -> impl WidgetView<AppState> {
    flex_col((
        label("TODO list"),
        label(format!(
            "{} items, {} done",
            state.todos.len(),
            state.todos.iter().filter(|todo| todo.done).count()
        )),
        sized_box(text_input(state.draft.clone(), |state: &mut AppState, draft| {
            state.draft = draft;
        }))
        .expand_width()
        .placeholder("Add a new task")
        .on_enter(|state: &mut AppState, draft| {
            state.draft = draft;
            state.add_todo();
        }),
        flex_row((
            text_button("Add", |state: &mut AppState| state.add_todo()),
            text_button("Clear input", |state: &mut AppState| {
                state.draft.clear();
            }),
        )),
        todo_rows(state),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Stretch)
}

// 状態 → ビュー の純粋な関数。状態が変わるたびに呼ばれ、UIが再構築される
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    flex_col((
        navigation(state),
        indexed_stack((home_page(state), todo_page(state))).active(state.page.index()),
    ))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState {
            page: Page::Home,
            count: 0,
            draft: String::new(),
            todos: vec![
                TodoItem {
                    title: "Learn how Xilem rebuilds views".to_string(),
                    done: false,
                },
                TodoItem {
                    title: "Add a second task".to_string(),
                    done: true,
                },
            ],
        },
        app_logic,
        WindowOptions::new("Xilem pages: counter + TODO list"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
