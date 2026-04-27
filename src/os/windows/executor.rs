use std::process::Command;

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    println!("▶ Running command: {} {:?}", cmd, args);

    let output = Command::new(cmd)
        .args(args)
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr).to_string();
                println!("❌ Command failed: {}", err);
                return Err(err);
            }

            let out = String::from_utf8_lossy(&output.stdout).to_string();
            println!("✅ Command success");
            Ok(out)
        }
        Err(e) => {
            println!("💥 Execution error: {}", e);
            Err(format!("Execution failed: {}", e))
        }
    }
}