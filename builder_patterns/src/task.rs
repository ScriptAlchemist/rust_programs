#[derive(Debug)]
pub struct Task {
    pub title: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            done: false,
            desc: None,
        }
    }
}

#[allow(dead_code)]
impl Task {
    pub fn new<T>(title: T) -> Task
    where
        T: Into<String>,
    {
        Task {
            title: title.into(),
            done: false,
            desc: None,
        }
    }
}

#[cfg(test)]
mod task_test {
    use super::*;

    #[test]
    fn default_same_module() {
        let task = Task::default();
        assert_eq!(task.title, "Untitled");
        assert_eq!(task.done, false);
        assert_eq!(task.desc, None);

        let task: Option<Task> = None;
        let task = task.unwrap_or_default();
        assert_eq!(task.title, "Untitled");
        assert_eq!(task.done, false);
        assert_eq!(task.desc, None);

        let task = Task {
            done: true,
            desc: Some(String::from("Walk the dog.")),
            ..Default::default()
        };
        assert_eq!(task.title, "Untitled");
        assert_eq!(task.done, true);
        assert_eq!(task.desc, Some(String::from("Walk the dog.")));
    }
    #[test]
    fn new_same_module() {
        let mut task = Task::new("Test One");
        assert_eq!(task.title, "Test One");
        assert_eq!(task.done, false);
        assert_eq!(task.desc, None);

        task.done = true;
        task.desc = Some(String::from("Kicking the soccer ball."));
        assert_eq!(task.done, true);
        assert_eq!(task.desc, Some(String::from("Kicking the soccer ball.")));

        let task = Task {
            desc: Some(String::from("Go for dinner")),
            ..Task::new("Date Night")
        };
        assert_eq!(task.title, "Date Night");
        assert_eq!(task.done, false);
        assert_eq!(task.desc.unwrap(), String::from("Go for dinner"));
    }
}
