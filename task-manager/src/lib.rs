use std::time::Instant;

struct Task {
    title: String,
    description: String,
    completed_at: Option<Instant>
}

impl Task {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: "".to_string(),
            completed_at: None,
        }
    }
}

pub trait Persisted<T> {
    fn create(t: &T)
}

#[cfg(test)]
mod tests {
    use crate::Task;

    #[test]
    fn it_works() {
        let expected_title = String::from("new Task");
        let task = Task::new(expected_title.clone());
        assert_eq!(task.title, expected_title);
        assert_eq!(task.description, "".to_string());
        assert_eq!(task.completed_at, None)
    }
}
