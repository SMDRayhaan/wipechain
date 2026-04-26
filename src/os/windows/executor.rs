use std::process::Command;

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    println!("Running command: {} {:?}", cmd, args);

    let output = Command::new(cmd)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            println!("Status: {:?}", output.status);

            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }

            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        }
        Err(e) => {
            Err(format!("Execution failed: {}", e))
        }
    }
}