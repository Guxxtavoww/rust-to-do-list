use serde::{ Serialize, Deserialize };
use serde_json::{ from_str, to_string };
use std::{ fs::File, io::{ Read, Write } };

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  id: u32,
  description: String,
  is_completed: bool,
}

pub fn load_tasks(file_path: &str) -> Vec<Task> {
  if !file_path.ends_with(".json") {
    panic!("File is not JSON")
  }

  let mut file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).expect("Falha ao criar o arquivo"));
  let mut file_content = String::new();

  file.read_to_string(&mut file_content).expect("Error");
  from_str(&file_content).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &Vec<Task>, file_path: &str) {
  let json = to_string(tasks).expect("Failed");
  let mut file = File::create(file_path).expect("Falha ao abrir ou criar o arquivo");

  file.write_all(json.as_bytes()).expect("Failed to write to file");
}

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
  let id: u32 = tasks.len() as u32 + 1;

  let new_task = Task {
    id,
    description,
    is_completed: false
  };

  tasks.push(new_task);
}

pub fn list_tasks(tasks: &Vec<Task>) {
  println!("--------------------------");

  for task in tasks {
    let status = if task.is_completed { "✔️" } else { "❌" };

    println!("{}: {} [{}]", task.id, task.description, status);
  }
}

pub fn toggle_task_completion(id: u32, tasks: &mut Vec<Task>) {
  if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
    task.is_completed = !task.is_completed;
  } else {
    println!("Task not found!");
  }
}

pub fn delete_task(id: u32, tasks: &mut Vec<Task>) {
  tasks.retain(|task| task.id != id);
}
