pub mod responds_build;
pub mod voice;

pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return "0 seconds".to_string();
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!(
            "{} hour{}",
            hours,
            if hours == 1 { "" } else { "s" }
        ));
    }
    if minutes > 0 {
        parts.push(format!(
            "{} minute{}",
            minutes,
            if minutes == 1 { "" } else { "s" }
        ));
    }
    if remaining_seconds > 0 {
        parts.push(format!(
            "{} second{}",
            remaining_seconds,
            if remaining_seconds == 1 { "" } else { "s" }
        ));
    }

    parts.join(" ")
}
