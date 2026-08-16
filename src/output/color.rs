pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

/// Applies an ANSI escape code to a string if color is enabled.
pub fn style(text: &str, ansi_code: &str, enabled: bool) -> String {
    if enabled && !ansi_code.is_empty() {
        format!("{}{}{}", ansi_code, text, RESET)
    } else {
        text.to_string()
    }
}

/// Applies bold styling to a string if color is enabled.
pub fn bold(text: &str, enabled: bool) -> String {
    if enabled {
        format!("{}{}{}", BOLD, text, RESET)
    } else {
        text.to_string()
    }
}

/// Formats a module label with bold primary color.
pub fn format_label(label: &str, primary_color: &str, enabled: bool) -> String {
    if enabled {
        format!("{}{}{}:{}", BOLD, primary_color, label, RESET)
    } else {
        format!("{}:", label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_enabled() {
        let s = style("hello", "\x1b[31m", true);
        assert_eq!(s, "\x1b[31mhello\x1b[0m");
    }

    #[test]
    fn test_style_disabled() {
        let s = style("hello", "\x1b[31m", false);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_bold_enabled() {
        let s = bold("text", true);
        assert_eq!(s, "\x1b[1mtext\x1b[0m");
    }

    #[test]
    fn test_bold_disabled() {
        let s = bold("text", false);
        assert_eq!(s, "text");
    }
}
