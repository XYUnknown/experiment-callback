/**
 * This is the call back function that will be called when the task is due.
 */
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CallBack {
    content: String
}

impl CallBack {
    pub fn new(content: String) -> CallBack {
        CallBack {
            content: content,
        }
    }
    pub fn execute(&self) {
        println!("Task {} is due", self.content);
    }
}