use std::{thread, time::Duration};

fn main() {
    loop {
        // Sleep for 20 minutes
        thread::sleep(Duration::from_secs(20 * 60));
        
        // Trigger notification
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("osascript")
                .args(&[
                    "-e",
                    "display notification \"Take a minute to look outside\" with title \"Look away from screen\"",
                ])
                .output()
                .expect("Failed to execute command");
        }
    }
}
