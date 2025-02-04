use std::env;
use std::process::Command;

fn main() {
    let mac = env::args()
        .nth(1)
        .expect("Please provide a MAC address as argument");
    const URL: &str = "http://dhcp.schule.local/cgi-bin/mypie.cgi?command=WLAN+Devices";

    // Use curl to perform the HTTP GET request.
    let output = Command::new("curl")
        .arg("-s")
        .arg(URL)
        .output()
        .expect("Failed to execute curl");

    let response_text = String::from_utf8_lossy(&output.stdout);

    if response_text.contains(&mac) {
        if let Some(index) = response_text.find(&mac) {
            // Split by MAC, then by "|" to obtain the IP address.
            let before_mac = &response_text[..index];
            let parts: Vec<&str> = before_mac.split('|').collect();
            if parts.len() >= 2 {
                let ip = parts[parts.len() - 2].trim();
                // Execute the SSH command.
                Command::new("ssh")
                    .arg(format!("pi@{}", ip))
                    .status()
                    .expect("Failed to execute ssh");
            }
        }
    }
}
