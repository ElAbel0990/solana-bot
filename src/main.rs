use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        let output1 = Command::new("sh")
            .arg("-c")
            .arg("solana airdrop --url devnet 1 AvGLCofJrZF8xrLn93M9ybuq18LFS53yHmUTauZo8bGV")
            .output()
            .expect("Failed to execute command");

        // Convert the output to a string and print it
        if let Ok(s) = String::from_utf8(output1.stdout) {
            println!("{}", s);
        } else {
            eprintln!("Failed to convert output to string");
        }

        // Second call to solana config get
        let output2 = Command::new("sh")
            .arg("-c")
            .arg("solana airdrop --url testnet 1 AvGLCofJrZF8xrLn93M9ybuq18LFS53yHmUTauZo8bGV")
            .output()
            .expect("Failed to execute command");

        // Convert the output to a string and print it
        if let Ok(s) = String::from_utf8(output2.stdout) {
            println!("{}", s);
        } else {
            eprintln!("Failed to convert output to string");
        }


        // Sleep for 24.25 hours
        thread::sleep(Duration::from_secs(87300));
    }
}
