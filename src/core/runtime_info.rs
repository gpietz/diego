use std::fmt::{Display, Formatter};

#[macro_export]
macro_rules! create_runtime_info {
    ($msg:expr) => {
        RuntimeInfo::new(
            file!(),
            module_path!(),
            line!(),
            Some($msg.to_string()),
        )
    };
    () => {
        RuntimeInfo::without_message(file!(), module_path!(), line!())
    };
}

#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub source_file: &'static str,
    pub module: &'static str,
    pub line: u32,
    pub message: Option<String>,
}

impl RuntimeInfo {
    pub fn new(source_file: &'static str, module: &'static str, line: u32, message: Option<String>) -> Self {
        RuntimeInfo { source_file, module, line, message }
    }

    pub fn without_message(source_file: &'static str, module: &'static str, line: u32) -> Self {
        Self::new(source_file, module, line, None)
    }
}

impl Display for RuntimeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.message {
            write!(f, "Error: {}\nFile: {}\nModule: {}\nLine: {}", msg, self.source_file, self.module, self.line)
        } else {
            write!(f, "File: {}\nModule: {}\nLine: {}", self.source_file, self.module, self.line)
        }
    }
}
