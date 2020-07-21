use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: ' '
        }
    }
}

struct TodoList {
    list: Vec<TodoItem>
}

impl TodoList {
    fn all() -> TodoList {
        return TodoList{list: Vec::new()};
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();

    let todo_list = TodoList::all();
    let todo_1 = TodoItem::new("Make a bread".to_string());
    let todo_2 = TodoItem::new("Brew ☕️".to_string());


    if command == "get" {
        for item in todo_list.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}
