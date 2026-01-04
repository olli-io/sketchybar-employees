use std::process::Command;

/// A builder for batching sketchybar commands
#[derive(Debug, Default)]
pub struct SketchybarBatch {
    args: Vec<String>,
}

impl SketchybarBatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set properties on an item
    pub fn set(&mut self, item: &str, props: &[(&str, &str)]) -> &mut Self {
        self.args.push("--set".to_string());
        self.args.push(item.to_string());
        for (key, value) in props {
            self.args.push(format!("{}={}", key, value));
        }
        self
    }

    /// Add animation with curve and duration
    pub fn animate(&mut self, curve: &str, duration: u32) -> &mut Self {
        self.args.push("--animate".to_string());
        self.args.push(curve.to_string());
        self.args.push(duration.to_string());
        self
    }

    /// Execute the batched commands
    pub fn execute(&self) -> Result<(), std::io::Error> {
        if self.args.is_empty() {
            return Ok(());
        }

        let status = Command::new("sketchybar")
            .args(&self.args)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "sketchybar command failed",
            ))
        }
    }

    /// Check if the batch has any commands
    #[allow(dead_code)] // Used in tests
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
}

/// Convenience function to set properties on a single item
pub fn set_item(item: &str, props: &[(&str, &str)]) -> Result<(), std::io::Error> {
    let mut batch = SketchybarBatch::new();
    batch.set(item, props);
    batch.execute()
}

/// Update the clock item
pub fn update_clock(time: &str) -> Result<(), std::io::Error> {
    set_item("clock", &[("label", time)])
}

/// Update the battery item
pub fn update_battery(icon: &str, percentage: u8) -> Result<(), std::io::Error> {
    set_item("battery", &[
        ("icon", icon),
        ("label", &format!("{}%", percentage)),
    ])
}

/// Update the volume item
pub fn update_volume(icon: &str, percentage: u8) -> Result<(), std::io::Error> {
    set_item("volume", &[
        ("icon", icon),
        ("label", &format!("{}%", percentage)),
    ])
}

/// Update the front app item
pub fn update_front_app(icon: &str, app_name: &str) -> Result<(), std::io::Error> {
    set_item("front_app", &[
        ("icon", icon),
        ("label", &format!("❯ {}", app_name)),
    ])
}

/// Update the brew outdated item
pub fn update_brew(icon: &str, formulae: usize, casks: usize) -> Result<(), std::io::Error> {
    let total = formulae + casks;
    let label = if total == 0 {
        "✓".to_string()
    } else {
        format!("{}", total)
    };
    set_item("brew", &[
        ("icon", icon),
        ("label", &label),
    ])
}

/// Update the Microsoft Teams notification item
pub fn update_teams(icon: &str, icon_color: &str, border_color: &str, notification_count: u32) -> Result<(), std::io::Error> {
    let label = if notification_count > 0 {
        format!("{}", notification_count)
    } else {
        String::new()
    };
    set_item("teams", &[
        ("icon", icon),
        ("icon.color", icon_color),
        ("background.border_color", border_color),
        ("label", &label),
        ("drawing", "on"),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_building() {
        let mut batch = SketchybarBatch::new();
        batch.set("clock", &[("label", "12:00")]);
        batch.set("battery", &[("icon", ""), ("label", "100%")]);

        assert!(!batch.is_empty());
        assert!(batch.args.contains(&"--set".to_string()));
        assert!(batch.args.contains(&"clock".to_string()));
        assert!(batch.args.contains(&"label=12:00".to_string()));
    }
}
