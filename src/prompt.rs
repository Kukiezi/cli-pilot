use inquire::{InquireError, Select};

pub enum UserOption {
    Run,
    Revise,
    Cancel,
}

impl UserOption {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "✅ Run this command" => Some(UserOption::Run),
            "📝 Revise query" => Some(UserOption::Revise),
            "❌ Cancel" => Some(UserOption::Cancel),
            _ => None,
        }
    }

    fn to_vec() -> Vec<&'static str> {
        vec!["✅ Run this command", "📝 Revise query", "❌ Cancel"]
    }
}

pub fn prompt_user() -> Result<&'static str, InquireError> {
    let options: Vec<&str> = UserOption::to_vec();
    let ans: Result<&str, InquireError> = Select::new("What you wanna do now?", options).prompt();
    ans
}
