use std::time::SystemTime;

#[derive(PartialEq)]
pub struct Task {
    title: String,
    description: String,
    completed_at: Option<SystemTime>,
}


impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: "".to_string(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        match self.completed_at {
            None => self.completed_at = Some(SystemTime::now()),
            x => self.completed_at = x
        }
    }
    pub fn uncomplete(&mut self) {
        self.completed_at = None
    }
}


pub trait Collection {
    type Task;

    fn new () -> Self;
    fn add_task(&mut self, task: Self::Task);
    fn remove_task(&mut self, task: Self::Task);
    fn get_all_tasks(&self) -> &Vec<Self::Task>;
}

pub struct TaskCollection {
    tasks: Vec<Task>,
}

impl Collection for TaskCollection {
    type Task = Task;

    fn new() -> Self {
        Self {
            tasks: vec![]
        }
    }
    fn add_task(&mut self, task: Self::Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, task: Self::Task){
        self.tasks.retain(|t| *t != task);
    }

    fn get_all_tasks(&self) -> &Vec<Self::Task> {
        &self.tasks
    }
}

#[cfg(test)]
mod tests {
    use crate::{Collection, Task, TaskCollection};

    #[test]
    fn it_inits_task() {
        let expected_title = String::from("new Task");
        let task = Task::new(expected_title.clone());
        assert_eq!(task.title, expected_title);
        assert_eq!(task.description, "".to_string());
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_completes_a_task() {
        let expected_title = String::from("new Task");
        let mut task = Task::new(expected_title.clone());
        task.complete();
        assert_ne!(task.completed_at, None)
    }

    #[test]
    fn it_uncompletes_a_task() {
        let expected_title = String::from("new Task");
        let mut task = Task::new(expected_title.clone());
        task.complete();
        task.uncomplete();
        assert_eq!(task.completed_at, None)
    }

    #[test]
    fn it_does_not_set_completes_mulitple_times() {
        let expected_title = String::from("new Task");
        let mut task = Task::new(expected_title.clone());
        task.complete();
        let expected_completed = task.completed_at;
        task.complete();
        assert_eq!(task.completed_at, expected_completed)
    }

    #[test]
    fn inits_empty_task_collection() {
        let collection = TaskCollection::new();
        assert_eq!(collection.tasks.is_empty(), true)
    }
}
