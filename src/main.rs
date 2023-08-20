use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use chrono::{NaiveDateTime, Local};

// Перечисление для приоритетов задач
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    High, 
    Medium,
    Low,  
}

impl Priority {
    // Метод, возвращающий цвет для каждого приоритета
    fn color(&self) -> ColorSpec {
        let mut color_spec = ColorSpec::new();
        match self {
            Priority::High => color_spec.set_fg(Some(Color::Red)),
            Priority::Medium => color_spec.set_fg(Some(Color::Yellow)),
            Priority::Low => color_spec.set_fg(Some(Color::Green)),
        };
        color_spec
    }
}

// Структура задачи
struct Task {
    description: String,
    completed: bool,
    priority: Priority,
    due_time: Option<NaiveDateTime>,
}

// Менеджер задач
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    // Создание нового экземпляра менеджера задач
    fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    // Добавление задачи в список с учетом приоритета
    fn add_task(&mut self, description: String, priority: Priority, due_time: Option<NaiveDateTime>) {
        self.tasks.push(Task { description, completed: false, priority, due_time });
        self.tasks.sort_by_key(|task| task.priority);
    }

    // Пометить задачу как завершенную по индексу
    fn complete_task(&mut self, index: usize) {
        self.tasks.get_mut(index).map(|task| task.completed = true);
    }

    // Вывод всех задач с приоритетами и статусами
    fn print_tasks(&self) {
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let mut stdout = stdout.lock();

        for (index, task) in self.tasks.iter().enumerate() {
            // Установка цвета для приоритета
            stdout.set_color(&task.priority.color()).unwrap();
            let status = if task.completed { "[x]" } else { "[ ]" };
            write!(stdout, "{} {}: {}", status, index, task.description).unwrap();

            if let Some(due_time) = task.due_time {
                let now = Local::now().naive_local();
                let (prefix, color) = if due_time > now {
                    let time_left = due_time - now;
                    (format!("Due in {} hours", time_left.num_hours()), Color::Green)
                } else {
                    ("Overdue".to_string(), Color::Red)
                };
                // Установка цвета для времени
                stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
                writeln!(stdout, " ({})", prefix).unwrap();
                stdout.reset().unwrap();
            } else {
                writeln!(stdout).unwrap();
            }
        }
    }
}

fn main() {
    // Создание экземпляра менеджера задач
    let mut task_manager = TaskManager::new();

    loop {
        print!("Enter a command (add/complete/print/quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "add" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let description = read_line();

                print!("Enter task priority (high/medium/low): ");
                io::stdout().flush().unwrap();
                let priority = match read_line().as_str() {
                    "high" => Priority::High,
                    "medium" => Priority::Medium,
                    "low" => Priority::Low,
                    _ => {
                        println!("Invalid priority. Using medium.");
                        Priority::Medium
                    }
                };

                print!("Enter due time (HH:MM dd-mm-yyyy) or leave empty: ");
                io::stdout().flush().unwrap();
                let due_time = read_line();
                let due_time = if due_time.is_empty() {
                    None
                } else {
                    match NaiveDateTime::parse_from_str(&due_time, "%H:%M %d-%m-%Y") {
                        Ok(datetime) => Some(datetime),
                        Err(_) => {
                            println!("Invalid date format. Leaving due time empty.");
                            None
                        }
                    }
                };

                // Добавление задачи с указанными данными
                task_manager.add_task(description, priority, due_time);
                println!("Task added.");
            }
            "complete" => {
                // Вывод текущих задач и запрос индекса завершенной задачи
                task_manager.print_tasks();
                print!("Enter task index to complete: ");
                io::stdout().flush().unwrap();
                if let Ok(index) = read_line().parse::<usize>() {
                    // Пометка задачи как завершенной по индексу
                    task_manager.complete_task(index);
                    println!("Task completed.");
                } else {
                    println!("Invalid index.");
                }
            }
            "print" => {
                // Вывод текущих задач
                task_manager.print_tasks();
            }
            "quit" => {
                // Завершение программы
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
}

// Чтение строки из ввода
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}