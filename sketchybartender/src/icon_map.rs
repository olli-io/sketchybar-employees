// Generated at compile time from icon_map.json
include!(concat!(env!("OUT_DIR"), "/icon_map.rs"));

/// Get the icon for an app name
pub fn get_icon(app_name: &str) -> &'static str {
    // First try exact match
    if let Some(icon) = ICON_MAP.get(app_name) {
        return icon;
    }

    // Try prefix patterns
    for (prefix, icon) in PREFIX_PATTERNS {
        if app_name.starts_with(prefix) {
            return icon;
        }
    }

    // Default icon
    ":default:"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert_eq!(get_icon("Cursor"), ":cursor:");
        assert_eq!(get_icon("Safari"), ":safari:");
        assert_eq!(get_icon("Discord"), ":discord:");
    }

    #[test]
    fn test_prefix_match() {
        assert_eq!(get_icon("Adobe Photoshop 2024"), ":photoshop:");
        assert_eq!(get_icon("MongoDB Compass Community"), ":mongodb:");
    }

    #[test]
    fn test_default() {
        assert_eq!(get_icon("Unknown App"), ":default:");
    }
}
