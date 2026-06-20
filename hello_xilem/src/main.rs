use xilem::view::{
    checkbox, flex_col, flex_row, indexed_stack, label, sized_box, text_button, text_input,
    virtual_scroll, CrossAxisAlignment,
};
use xilem::winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Todo,
    Form,
}

impl Page {
    fn index(self) -> usize {
        match self {
            Page::Home => 0,
            Page::Todo => 1,
            Page::Form => 2,
        }
    }
}

#[derive(Clone)]
struct TodoItem {
    id: u64,
    title: String,
    done: bool,
}

// アプリ全体の状態を1つの構造体で管理する
struct AppState {
    page: Page,
    count: i32,
    draft: String,
    name_input: String,
    todos: Vec<TodoItem>,
    next_todo_id: u64,
}

impl AppState {
    fn add_todo(&mut self) {
        let title = self.draft.trim();
        if title.is_empty() {
            return;
        }

        self.todos.push(TodoItem {
            id: self.next_todo_id,
            title: title.to_owned(),
            done: false,
        });
        self.next_todo_id += 1;
        self.draft.clear();
    }

    fn toggle_todo(&mut self, id: u64, done: bool) {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.id == id) {
            todo.done = done;
        }
    }

    fn remove_todo(&mut self, id: u64) {
        self.todos.retain(|todo| todo.id != id);
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
        text_button("Form", |state: &mut AppState| {
            state.page = Page::Form;
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
        let todo = state.todos.get(idx as usize).cloned();
        let (title, done, todo_id) = match todo {
            Some(todo) => (todo.title, todo.done, Some(todo.id)),
            None => ("(stale item)".to_string(), false, None),
        };

        flex_row((
            checkbox(title, done, move |state: &mut AppState, checked| {
                if let Some(id) = todo_id {
                    state.toggle_todo(id, checked);
                }
            }),
            text_button("Delete", move |state: &mut AppState| {
                if let Some(id) = todo_id {
                    state.remove_todo(id);
                }
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
        sized_box(
            text_input(state.draft.clone(), |state: &mut AppState, draft| {
            state.draft = draft;
        })
            .placeholder("Add a new task")
            .on_enter(|state: &mut AppState, draft| {
                state.draft = draft;
                state.add_todo();
            }),
        )
        .expand_width(),
        flex_row((
            text_button("Add", |state: &mut AppState| state.add_todo()),
            text_button("Clear input", |state: &mut AppState| {
                state.draft.clear();
            }),
        )),
        todo_rows(state),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Fill)
}

fn validate_name(name: &str) -> (&'static str, String) {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return ("Please enter your name.", String::new());
    }

    if trimmed.chars().count() < 2 {
        return ("Name is too short (min 2 chars).", String::new());
    }

    if !trimmed
        .chars()
        .all(|c| c.is_alphabetic() || c == ' ' || c == '-' || c == '\'')
    {
        return (
            "Use letters, spaces, apostrophe, or hyphen only.",
            String::new(),
        );
    }

    ("Looks good!", format!("Hello, {}!", trimmed))
}

fn form_page(state: &mut AppState) -> impl WidgetView<AppState> {
    let (validation_message, greeting) = validate_name(&state.name_input);

    flex_col((
        label("Name form + validation"),
        sized_box(
            text_input(state.name_input.clone(), |state: &mut AppState, value| {
                state.name_input = value;
            })
            .placeholder("Type your name"),
        )
        .expand_width(),
        label(validation_message),
        label(greeting),
        text_button("Clear", |state: &mut AppState| {
            state.name_input.clear();
        }),
    ))
    .cross_axis_alignment(CrossAxisAlignment::Fill)
}

// 状態 → ビュー の純粋な関数。状態が変わるたびに呼ばれ、UIが再構築される
fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    flex_col((
        navigation(state),
        indexed_stack((home_page(state), todo_page(state), form_page(state))).active(state.page.index()),
    ))
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState {
            page: Page::Home,
            count: 0,
            draft: String::new(),
            name_input: String::new(),
            todos: vec![
                TodoItem {
                    id: 0,
                    title: "Learn how Xilem rebuilds views".to_string(),
                    done: false,
                },
                TodoItem {
                    id: 1,
                    title: "Add a second task".to_string(),
                    done: true,
                },
            ],
            next_todo_id: 2,
        },
        app_logic,
        WindowOptions::new("Xilem pages: counter + TODO + form"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
