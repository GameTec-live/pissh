use std::env;
use std::process::Command;

fn main() {
    let mac = env::args()
        .nth(1)
        .expect("Please provide a MAC address as argument");
    const URL: &str = "http://dhcp.schule.local/cgi-bin/mypie.cgi?command=WLAN+Devices";

    // Use reqwest for HTTP request
    let response_text = reqwest::blocking::get(URL)
        .expect("Failed to make request")
        .text()
        .expect("Failed to get response text");

    if response_text.contains(&mac) {
        if let Some(index) = response_text.find(&mac) {
            // Split by MAC, then by "|" to obtain the IP address
            let before_mac = &response_text[..index];
            let parts: Vec<&str> = before_mac.split('|').collect();
            if parts.len() >= 2 {
                let ip = parts[parts.len() - 2].trim();
                // print the IP address
                println!("Found pi at {}", ip);
                // Execute the SSH command
                Command::new("ssh")
                    .arg(format!("pi@{}", ip))
                    .status()
                    .expect("Failed to execute ssh");
            }
        }
    } else {
        eprintln!("MAC address not found in response");
    }
}
