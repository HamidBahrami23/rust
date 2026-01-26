
// ## 🦀 Mini Project Exercise: **Simple Task Manager (CLI logic only)**

#[derive(Debug)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: u32 , title: String) -> Self {
        Self {
            id,
            title,
            status: TaskStatus::Todo,
        }
    }
    fn change_task_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }
}


fn find_task_by_id(tasks: &Vec<Task> , id: u32) -> Option<&Task> {
    for task in tasks {
        if task.id == id {
            return Some(task);
        } 
    }
    None
}



fn print_task(tasks: &Vec<Task>, id: u32) {
    let export = find_task_by_id(tasks, id);
    match export {
        Some(exp) => println!("id task: {} , name task: {} , stat: {:?} ",exp.id,exp.title , exp.status ),
        None => println!("id task: {} not found" , id),
    }
}



fn update_status(tasks: &mut Vec<Task> , id: u32 , stat: TaskStatus){
    let mut found = false;
    for task in tasks {
        if task.id == id {
            task.change_task_status(stat);
            found = true;
            break;
        }
    }
    match found {
        true => println!("id task: {} changed stat", id),
        false => println!("id task: {} not found!", id),
    }
}


fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task::new(01, "Learn Rust".to_owned()));
    tasks.push(Task::new(02, "Learn English".to_owned()));
    tasks.push(Task::new(03, "Learn Linux".to_owned()));
    tasks.push(Task::new(04, "Learn ML".to_owned()));

    print_task(&tasks, 01);
    print_task(&tasks, 02);
    print_task(&tasks, 03);
    print_task(&tasks, 04);
    print_task(&tasks, 09);

    update_status(&mut tasks, 04, TaskStatus::InProgress);
    update_status(&mut tasks, 01, TaskStatus::Done);
    update_status(&mut tasks, 09, TaskStatus::InProgress);

    print_task(&tasks, 01);
    print_task(&tasks, 02);
    print_task(&tasks, 03);
    print_task(&tasks, 04);

}


