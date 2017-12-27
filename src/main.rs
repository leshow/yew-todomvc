#![recursion_limit="128"]

#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    todos: Vec<Todo>,
    filter: Filter,
    field: String,
}

#[derive(PartialEq, Clone)]
enum Filter {
    All,
    Active,
    Completed,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match *self {
            Filter::All => "all".into(),
            Filter::Active => "active".into(),
            Filter::Completed => "completed".into()
        }
    }
}

struct Todo {
    id: usize,
    body: String,
    status: Status,
}

#[derive(PartialEq)]
enum Status {
    Completed,
    Active,
}

enum Msg {
    Add,
    ToggleAll,
    Toggle(usize),
    Update(String),
    Remove(usize),
    ChangeFilter(Filter),
    RemoveCompleted,
    Nil,
}

use self::Msg::*;

fn main() {
    let model = Model::new();
    program(model, update, view);
}

fn update(ctx: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Add => {}
        ToggleAll => {}
        Toggle(id) => {}
        Update(content) => {}
        Remove(id) => {}
        ChangeFilter(filter) => {}
        RemoveCompleted => {}
        Nil => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div class="todomvc-wrapper",>
            <section class="todoapp",>
                <header class="header",>
                    <h1> {"todos"} </h1>
                </header>
            <section class="main",>
                <input class="toggle-all",
                    onclick=|_| Msg::ToggleAll,
                    checked=model.all_completed(),/>
                <ul class="todo-list",>
                    { for model.todos.iter().enumerate().map(todo_html) }
                </ul>
            </section>
                <footer class="footer",>
                    <span class="todo-count",>
                        <strong></strong> { " item(s) left" }
                    </span>
                    <ul class="filters",>
                        { filter_html(&model, Filter::All) }
                        { filter_html(&model, Filter::Active) }
                        { filter_html(&model, Filter::Completed) }
                    </ul>
                    <button class="clear-completed",>
                    </button>
                </footer>
            </section>
            <footer class="info",>
                <p>{ "Double-click to edit a todo" }</p>
                <p>{ "Part of " }<a href="http://todomvc.com/", target="_blank",>{ "TodoMVC" }</a></p>
            </footer>
        </div>
    }
}

fn filter_html(model: &Model, filter: Filter) -> Html<Msg> {
    let fstring = filter.to_string();
    html! {
        <li>
            <a class= if model.filter == filter { "selected" } else { "not-selected" },
                href=&filter,
                onclick=move |_| ChangeFilter(filter.clone()),
            >
            { fstring }
        </li>
    }
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => Href::from("#/"),
            Filter::Active => Href::from("#/active"),
            Filter::Completed => Href::from("#/completed"),
        }
    }
}

fn input_html((idx, model): (usize, &Model)) -> Html<Msg> {
    html! {
        <input class="new-todo",
            placeholder="What needs to be done?",
            value=&model.field,
            oninput=|e: InputData| Update(e.value),
            onkeypress=|e: KeyData| {
                if e.key == "Enter" { Add } else { Nil }
            },
        />
    }
}

fn todo_html((idx, todo): (usize, &Todo)) -> Html<Msg> {
    html! {
        <li>
            <div class="view",>
                <input class="toggle",
                    type="checkbox",
                    checked=(todo.status == Status::Completed),
                    onclick=move|_| Toggle(idx),
                />
                <label>{ &todo.body }</label>
                <button class="destroy",
                    onclick=move |_| Remove(idx),/>
            </div>
        </li>
    }
}

impl Model {
    fn new() -> Model {
        Model {
            todos: Vec::new(),
            filter: Filter::All,
            field: "".into(),
        }
    }

    fn all_completed(&self) -> bool {
        self.todos.iter().filter(|t| t.status == Status::Completed).count() == self.todos.len()
    }
}