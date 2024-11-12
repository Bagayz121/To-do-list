use std::io::{self, stdin};

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        
        println!("Выберите действие:");
        println!("1. Добавить задачу");
        println!("2. Удалить задачу");
        println!("3. Просмотреть задачи");
        println!("4. Отметить задачу как выполненную");
        println!("5. Выход");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Ошибка чтения строки");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Введите описание задачи.");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Ошибка чтения строки");
                add_task(&mut tasks, description.trim().to_string());
            }

            2 => {
                println!("Введите индекс задачи для удаления:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Ошибка чтения строки");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный индекс!");
                        continue;
                    }
                };
                remove_task(&mut tasks, index);
            }

            3 => {
                view_task(&tasks);
            }

            4 => {
                println!("Введите индекс задачи для удаления:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Ошибка чтения строки");
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Неверный индекс!");
                        continue;
                    }
                };
                complete_task(&mut tasks, index);
            }

            5 => {
                println!("Выход из программы!");
                break;
            }

            _ => {
                println!("Неверный выбор. Попробуйте снова.");
            }
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    tasks.push (Task{
        description,
        completed: false,
    });
}

fn remove_task(tasks: &mut Vec<Task>, index: usize){
    if index < tasks.len() {
        tasks.remove(index);
    } else {
        println!("Задачи с таким индексом не существует!");
    }
}

fn view_task (tasks: &Vec <Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed {"Выполнено"} else {"Не выполнено"};
        println!("{}. [{}] {}", index, status, task.description);
    }
}

fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if index < tasks.len() {
        tasks[index].completed = true;
    } else {
        println!("Задачи с таким индексом не существует!");
    }
}

