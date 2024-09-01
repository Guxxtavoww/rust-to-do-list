use std::io::stdin;

pub fn show_menu() {
    println!("1. Adicionar Tarefa");
    println!("2. Listar Tarefas");
    println!("3. Trocar status da tarefa");
    println!("4. Remover Tarefa");
    println!("5. Sair & Salvar \n\n");
}

pub fn get_task_id_from_input(message: &str) -> u32 {
    println!("{}", message);
    let mut id = String::new();

    stdin().read_line(&mut id).expect("Failed to read line");

    let parsed_id: u32 = id.trim().parse().expect("Invalid ID");

    return parsed_id
}
