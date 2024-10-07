use chrono::Local;
use crate::logging::log_manager::LogMessage;

pub(crate) const DEFAULT_FORMAT : &str = "${longdate} [${level}] ${message}";

pub(crate) fn format_message(format: &str, message: &LogMessage) -> String {
    let now = Local::now();
    format.replace("${longdate}", &now.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
            .replace("${shortdate}", &now.format("%Y-%m-%d").to_string())
            .replace("${time}", &now.format("%H:%M:%S%.3f").to_string())
            .replace("${ticks}", &now.timestamp_nanos_opt().unwrap().to_string())
            .replace("${level}", &message.level.to_string())
            .replace("${message}", &message.message.to_string())
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use crate::logging::LogLevel;
    use super::*;

    #[test]
    fn test_format_message_with_longdate() {
        let log_message = LogMessage::new(LogLevel::Info, "Test message");
        let format = "${longdate} [${level}] ${message}";
        let result = format_message(format, &log_message);

        // Check whether the format is replaced correctly
        assert!(result.contains("INFO"));
        assert!(result.contains("Test message"));

        // Check whether the timestamp is in the correct format
        let regex = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}").unwrap();
        assert!(regex.is_match(&result), "Timestamp format is incorrect");
    }

    #[test]
    fn test_format_message_with_shortdate() {
        let log_message = LogMessage::new(LogLevel::Error, "An error occurred");
        let format = "${shortdate} [${level}] ${message}";
        let result = format_message(format, &log_message);

        // Check whether the date, level and message are formatted correctly
        assert!(result.contains("ERROR"));
        assert!(result.contains("An error occurred"));

        // Check whether the timestamp is in the correct format
        let regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        assert!(regex.is_match(&result), "Short date format is incorrect");
    }

    #[test]
    fn test_format_message_with_time() {
        let log_message = LogMessage::new(LogLevel::Debug, "Debugging");
        let format = "${time} [${level}] ${message}";
        let result = format_message(format, &log_message);

        // Check whether the time, level and message are formatted correctly
        assert!(result.contains("DEBUG"));
        assert!(result.contains("Debugging"));

        // Check whether the timestamp is in the correct format
        let regex = Regex::new(r"\d{2}:\d{2}:\d{2}\.\d{3}").unwrap();
        assert!(regex.is_match(&result), "Time format is incorrect");
    }

    #[test]
    fn test_format_message_with_ticks() {
        let log_message = LogMessage::new(LogLevel::Warn, "Warning issued");


        let format = "${ticks} [${level}] ${message}";
        let result = format_message(format, &log_message);

        // Check whether the level and the message are formatted correctly
        assert!(result.contains("WARN"));
        assert!(result.contains("Warning issued"));

        // Check whether the ticks are formatted correctly
        let regex = Regex::new(r"\d+").unwrap();
        assert!(regex.is_match(&result), "Ticks format is incorrect");
    }
}
