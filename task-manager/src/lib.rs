use std::time::SystemTime;

struct Task {
    title: String,
    description: String,
    completed_at: Option<SystemTime>
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
        self.completed_at = Some(SystemTime::now())
    }
}

#[cfg(test)]
mod tests {
    use crate::Task;

    #[test]
    fn it_inits_task() {
        let expected_title = String::from("new Task");
        let task = Task::new(expected_title.clone());
        assert_eq!(task.title, expected_title);
        assert_eq!(task.description, "".to_string());
        assert_eq!(task.completed_at, None)
    }
    #[test]
    fn it_completes_a_task(){
        let expected_title = String::from("new Task");
        let mut task = Task::new(expected_title.clone());
        task.complete();
        assert_ne!(task.completed_at, None)
    }
}
