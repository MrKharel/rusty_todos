use serde::{Deserialize, Serialize};
use std::{fs, io};

const FILE: &str = "todos.json";

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

fn main() {
    let mut todos = load_todos();

    loop {
        let mut raw = String::new();
        io::stdin().read_line(&mut raw).expect("failed to read input");

        // strip only the line terminator, nothing else
        let line = raw.strip_suffix('\n').unwrap_or(&raw);
        let line = line.strip_suffix('\r').unwrap_or(line);

        if let Some(title) = line.strip_prefix("todos add ") {
            add_todo(&mut todos, title);
        } else if let Some(id) = line.strip_prefix("todos remove ") {
            remove_todo(&mut todos, id);
        } else if let Some(id) = line.strip_prefix("todos done ") {
            done_todo(&mut todos, id);
        } else if line != "todos" {
            println!("Invalid. Commands:");
            println!("  todos add <title>");
            println!("  todos remove <id>");
            println!("  todos done <id>");
            return;
        }
        print_todos(&todos);
    }
}

fn load_todos() -> Vec<Todo> {
    match fs::read_to_string(FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("{} is not valid JSON: {}", FILE, e);
            std::process::exit(1);
        }),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
        Err(e) => {
            eprintln!("could not read {}: {}", FILE, e);
            std::process::exit(1);
        }
    }
}

fn save_todos(todos: &[Todo]) {
    let json = serde_json::to_string_pretty(todos).expect("could not serialize todos");
    fs::write(FILE, json).expect("could not write todos file");
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    let id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    todos.push(Todo { id, title: title.to_string(), done: false });
    save_todos(todos);
}

fn remove_todo(todos: &mut Vec<Todo>, id: &str) {
    let Ok(id) = id.parse::<u32>() else {
        println!("Invalid id: {:?}", id);
        return;
    };
    let before = todos.len();
    todos.retain(|t| t.id != id);
    if todos.len() == before {
        println!("No todo with id {}", id);
        return;
    }
    save_todos(todos);
}

fn done_todo(todos: &mut Vec<Todo>, id: &str) {
    let Ok(id) = id.parse::<u32>() else {
        println!("Invalid id: {:?}", id);
        return;
    };
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            todo.done = true;
            save_todos(todos);
        }
        None => println!("No todo with id {}", id),
    }
}

fn print_todos(todos: &[Todo]) {
    for todo in todos {
        println!("[{}] {}: {}", if todo.done { "x" } else { " " }, todo.id, todo.title);
    }
}
