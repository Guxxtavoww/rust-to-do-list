mod task;
mod utils;

use std::io::stdin;
use utils::{ show_menu, get_task_id_from_input };
use task::{ add_task, delete_task, list_tasks, load_tasks, save_tasks, toggle_task_completion  };

const INPUTS: [&str; 5] = ["1", "2", "3", "4", "5"];
const FILE_PATH: &str = "tasks.json";

fn main() {
    let mut tasks = load_tasks(FILE_PATH);

    loop {
        show_menu();

        let mut choice = String::new();
        stdin().read_line(&mut choice).expect("Erro read line");

        let trimmed_choice = choice.trim();

        if !INPUTS.contains(&trimmed_choice) {
            println!("Opção inválida! Tente novamente.");

            continue;
        }

        match trimmed_choice {
            "1" => {
                println!("Digite a descrição da tarefa:");
                let mut description = String::new();
                std::io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();
                add_task(&mut tasks, description);
            },
            "2" => {
                println!("Tarefas:");
                list_tasks(&tasks);
            },
            "3" => {
                let parsed_id: u32 = get_task_id_from_input("Digite o ID da tarefa para mudar o status:");
                toggle_task_completion(parsed_id, &mut tasks)
            },
            "4" => {
                let parsed_id: u32 = get_task_id_from_input("Digite o ID da tarefa a ser removida:");
                delete_task(parsed_id, &mut tasks)
            },
            "5" => {
                save_tasks(&tasks, FILE_PATH);
                break;
            },
            _ => println!("Opção inválida!"),
        }
    }
}
