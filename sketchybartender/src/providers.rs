use std::process::Command;

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub charging: bool,
}

impl BatteryInfo {
    /// Get the appropriate icon for the battery state
    pub fn icon(&self) -> &'static str {
        if self.charging {
            return "\u{f0e7}"; // nf-md-battery_charging_50
        }
        match self.percentage {
            90..=100 => "\u{f240}", // nf-md-battery_high
            70..=89 => "\u{f241}",  // nf-md-battery_medium
            40..=69 => "\u{f242}",  // nf-md-battery_medium
            10..=39 => "\u{f243}",  // nf-md-battery_low
            _ => "\u{f244}",        // nf-md-battery_outline
        }
    }
}

/// Get current battery information
pub fn get_battery() -> Option<BatteryInfo> {
    let output = Command::new("pmset")
        .args(["-g", "batt"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Parse percentage - look for word containing '%' (e.g., "26%;" or "100%")
    let percentage = stdout
        .split_whitespace()
        .find(|s| s.contains('%'))
        .and_then(|s| {
            // Extract digits before the '%' sign
            s.split('%').next()?.parse::<u8>().ok()
        })?;

    // Check if charging
    let charging = stdout.contains("AC Power");

    Some(BatteryInfo { percentage, charging })
}

/// Volume information
#[derive(Debug, Clone)]
pub struct VolumeInfo {
    pub percentage: u8,
    pub muted: bool,
}

impl VolumeInfo {
    /// Get the appropriate icon for the volume level
    pub fn icon(&self) -> &'static str {
        if self.muted || self.percentage == 0 {
            return "󰖁";
        }
        match self.percentage {
            60..=100 => "󰕾",
            30..=59 => "󰖀",
            _ => "󰕿",
        }
    }
}

/// Get current volume information
pub fn get_volume() -> Option<VolumeInfo> {
    let output = Command::new("osascript")
        .args(["-e", "output volume of (get volume settings)"])
        .output()
        .ok()?;

    let volume_str = String::from_utf8_lossy(&output.stdout);
    let percentage = volume_str.trim().parse::<u8>().ok()?;

    // Check mute status
    let mute_output = Command::new("osascript")
        .args(["-e", "output muted of (get volume settings)"])
        .output()
        .ok()?;

    let muted = String::from_utf8_lossy(&mute_output.stdout)
        .trim()
        .eq_ignore_ascii_case("true");

    Some(VolumeInfo { percentage, muted })
}

/// Get current time formatted as DD/MM HH:MM
pub fn get_clock() -> String {
    // Use shell command to avoid pulling in chrono dependency
    let output = Command::new("date")
        .args(["+%d/%m %H:%M"])
        .output();
    
    match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => "??/?? ??:??".to_string()
    }
}


/// Brew outdated information
#[derive(Debug, Clone, Default)]
pub struct BrewInfo {
    pub formulae: usize,
    pub casks: usize,
}

impl BrewInfo {
    /// Get the total count of outdated packages
    pub fn total(&self) -> usize {
        self.formulae + self.casks
    }

    /// Get the appropriate icon
    pub fn icon(&self) -> &'static str {
        if self.total() == 0 {
            "󰏗" // nf-md-package_variant (checkmark style)
        } else {
            "󰏔" // nf-md-package_variant_closed (needs attention)
        }
    }
}

/// Get outdated brew formulae and casks count
pub fn get_brew_outdated() -> BrewInfo {
    let mut info = BrewInfo::default();

    // Get outdated formulae
    if let Ok(output) = Command::new("brew")
        .args(["outdated", "--formula", "-q"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            info.formulae = stdout.lines().filter(|l| !l.is_empty()).count();
        }
    }

    // Get outdated casks
    if let Ok(output) = Command::new("brew")
        .args(["outdated", "--cask", "-q"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            info.casks = stdout.lines().filter(|l| !l.is_empty()).count();
        }
    }

    info
}

/// CPU and RAM usage information
#[derive(Debug, Clone, Default)]
pub struct SystemInfo {
    pub cpu_percentage: u8,
    pub ram_percentage: u8,
}

impl SystemInfo {
    /// Get the appropriate CPU icon based on usage
    pub fn cpu_icon(&self) -> &'static str {
        match self.cpu_percentage {
            80..=100 => "󰻠", // nf-md-cpu_high
            50..=79 => "󰻟",  // nf-md-cpu_medium
            _ => "󰘚",       // nf-md-cpu_low
        }
    }

    /// Get the appropriate RAM icon based on usage
    pub fn ram_icon(&self) -> &'static str {
        match self.ram_percentage {
            80..=100 => "󰍛", // nf-md-memory_high
            50..=79 => "󰍛",  // nf-md-memory_medium
            _ => "󰍛",       // nf-md-memory_low
        }
    }
}

/// Microsoft Teams notification information
#[derive(Debug, Clone, Default)]
pub struct TeamsInfo {
    pub running: bool,
    pub notification_count: u32,
}

impl TeamsInfo {
    /// Get the appropriate icon (Microsoft Teams icon)
    pub fn icon(&self) -> &'static str {
        "󰊻" // nf-md-microsoft_teams
    }

    /// Get the icon color based on state
    pub fn icon_color(&self) -> &'static str {
        if !self.running {
            "0xff3c3836" // Same as active workspace bg when not running
        } else if self.notification_count > 0 {
            "0xfffabd2f" // Yellow/amber when notifications
        } else {
            "0xffffffff" // White (same as other icons)
        }
    }

    /// Get the border color based on state
    pub fn border_color(&self) -> &'static str {
        if self.notification_count > 0 {
            "0xfffabd2f" // Yellow/amber border for notifications
        } else {
            "0xff2a2c3a" // Default border
        }
    }
}

/// Get Microsoft Teams notification count
pub fn get_teams_notifications() -> TeamsInfo {
    let mut info = TeamsInfo::default();

    // Check if Teams is running (MSTeams is the new Teams app process name)
    let running = Command::new("pgrep")
        .args(["-x", "MSTeams"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    info.running = running;

    if !running {
        return info;
    }

    // Get notification count from Dock badge via AppleScript
    let script = r#"
tell application "System Events"
    tell UI element "Microsoft Teams" of list 1 of process "Dock"
        try
            set badgeValue to value of attribute "AXStatusLabel"
            if badgeValue is not missing value then
                return badgeValue
            end if
        end try
    end tell
end tell
return "0"
"#;

    if let Ok(output) = Command::new("osascript")
        .args(["-e", script])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Extract only digits from the result
            let count_str: String = stdout.trim().chars().filter(|c| c.is_ascii_digit()).collect();
            info.notification_count = count_str.parse().unwrap_or(0);
        }
    }

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_icons() {
        let high = BatteryInfo { percentage: 95, charging: false };
        assert_eq!(high.icon(), "󱊣");

        let charging = BatteryInfo { percentage: 50, charging: true };
        assert_eq!(charging.icon(), "\u{f0e7}"); // nf-fa-bolt

        let low = BatteryInfo { percentage: 5, charging: false };
        assert_eq!(low.icon(), "󰂎");
    }

    #[test]
    fn test_volume_icons() {
        let high = VolumeInfo { percentage: 80, muted: false };
        assert_eq!(high.icon(), "\u{f240}");

        let muted = VolumeInfo { percentage: 80, muted: true };
        assert_eq!(muted.icon(), "󰖁");

        let zero = VolumeInfo { percentage: 0, muted: false };
        assert_eq!(zero.icon(), "\u{f244}");
    }

    #[test]
    fn test_clock() {
        let clock = get_clock();
        assert!(clock.contains('/'));
        assert!(clock.contains(':'));
    }
}
