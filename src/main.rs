#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    todos: Vec<Todo>,
    filter: Filter,
    field: String,
}

#[derive(PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

struct Todo {
    id: i32,
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
    println!("Hello, world!");
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
                    {input_html(&model)}
                </header>
            <section class="main",>
                <input class="toggle-all",/>
                <ul class="todo-list",>
                </ul>
            </section>
                <footer class="footer",>
                    <span class="todo-count",>
                        <strong></strong> { " item(s) left" }
                    </span>
                    <ul class="filters",>
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

fn input_html(model: &Model) -> Html<Msg> {
    html! {
        <input class="new-todo",
            placeholder="What needs to be done?",
            value=&model.field,
        />
    }
}