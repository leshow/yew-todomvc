#[macro_use]
extern crate yew;

use yew::*;

struct Model {
    todos: Vec<Todo>,
    filter: Filter,
    field: String,
}

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

enum Status {
    Completed,
    Active,
}

enum Msg {
    Add,
    Mark,
}


fn main() {
    println!("Hello, world!");
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div class="todomvc-wrapper",>
        </div>
    }
}
