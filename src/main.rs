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

    fn add_new(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn print(&self) {
        for item in &self.list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String)
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = match arguments[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        _ => panic!("You must provide an accepted command")
    };


    let mut todo_list = TodoList::all();
    todo_list.add_new("Make a bread".to_string());
    todo_list.add_new("Brew ☕️".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_new(task);
            todo_list.print();
        }
    }
}
