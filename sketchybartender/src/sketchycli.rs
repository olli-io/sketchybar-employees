//! CLI tool to replace shell scripts - sends messages to the daemon or handles direct actions

use std::env;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::Command;

fn get_socket_path() -> PathBuf {
    let cache_dir = env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".cache")
        });

    cache_dir.join("sketchybar").join("helper.sock")
}

fn send_message(message: &str) {
    let socket_path = get_socket_path();
    match UnixStream::connect(&socket_path) {
        Ok(mut stream) => {
            if let Err(e) = writeln!(stream, "{}", message) {
                eprintln!("Failed to send message '{}': {}", message, e);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to daemon at {:?}: {}", socket_path, e);
            eprintln!("Is sketchybartender daemon running?");
        }
    }
}

fn print_usage() {
    eprintln!(
        "Usage: sketchycli <command> [args...]

Commands:
  on-brew-clicked      - Trigger brew upgrade
  on-focus-change [app] - Trigger front app update (app from args or $INFO)
  send <message>       - Send arbitrary message to daemon
  on-teams-clicked     - Open Microsoft Teams
  on-volume-change [level] - Trigger volume update (level from args or $INFO)
  on-workspace-change  - Trigger workspace update
  on-workspace-clicked - Navigate to workspace (uses $NAME, $BUTTON)

Note: Clock, battery, brew, and teams updates are now handled automatically
      by the sketchybartender daemon. Update intervals can be configured in
      ~/.config/sketchybar/sketchybartenderrc"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "on-brew-clicked" => {
            send_message("brew-upgrade");
        }

        "on-focus-change" => {
            send_message("focus-change");
        }

        "on-teams-clicked" => {
            // Open Microsoft Teams (or bring to front if already running)
            let _ = Command::new("open")
                .arg("-a")
                .arg("Microsoft Teams")
                .spawn();

            // Immediate refresh to show responsiveness
            send_message("teams");

            // Refresh multiple times to catch state changes:
            // - Process start/stop (teams launching or quitting)
            // - Notification count changes (teams marking as read)
            std::thread::spawn(|| {
                // Refresh at 1s (catch quick process start)
                std::thread::sleep(std::time::Duration::from_secs(1));
                send_message("teams");

                // Refresh at 3s (process should be fully started by now)
                std::thread::sleep(std::time::Duration::from_secs(2));
                send_message("teams");

                // Refresh at 6s (notifications should be cleared by now)
                std::thread::sleep(std::time::Duration::from_secs(3));
                send_message("teams");

                // Final refresh at 10s (ensure all state changes are captured)
                std::thread::sleep(std::time::Duration::from_secs(4));
                send_message("teams");
            });
        }

        "on-volume-change" => {
            // Get volume level from args or $INFO environment variable
            let vol = args.get(2)
                .map(|s| s.to_string())
                .or_else(|| env::var("INFO").ok());

            if let Some(v) = vol {
                send_message(&format!("volume {}", v));
            } else {
                send_message("volume");
            }
        }

        "on-workspace-change" => {
            send_message("workspace-change");
        }

        "on-workspace-clicked" => {
            // Extract workspace ID from NAME (e.g., "workspace.3" -> "3")
            let name = env::var("NAME").unwrap_or_default();
            let button = env::var("BUTTON").unwrap_or_default();

            if button == "left" {
                if let Some(workspace_id) = name.strip_prefix("workspace.") {
                    let _ = Command::new("aerospace")
                        .arg("workspace")
                        .arg(workspace_id)
                        .spawn();
                }
            }
        }

        "send" => {
            // Send arbitrary message to daemon
            if args.len() < 3 {
                eprintln!("Error: 'send' command requires a message argument");
                print_usage();
                std::process::exit(1);
            }
            let message = args[2..].join(" ");
            send_message(&message);
        }

        "help" | "--help" | "-h" => {
            print_usage();
        }

        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}
