use std::io::{self, Write};
use std::process::Command;
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    let mut code: String = String::new();
    code.push_str("fn main() {");

    println!("Enter your Rust code:");
    loop {
        let file_path = "_temp_main.rs";
        let exec_path = Path::new("./").join(Path::new(file_path).file_stem().unwrap());
        let mut line = String::new();
        // Read code from the terminal until double enter is entered
        loop {
            print!(">>> ");
            let _ = io::stdout().flush();
            io::stdin().read_line(&mut line)?;
            if line.trim() == "" {
                let mut fin_code: String = code.clone();
                // Add ending
                fin_code.push_str("}");

                // Write the code to a file
                let mut file = File::create(file_path)?;
                file.write_all(fin_code.as_bytes())?;

                // Compile the code
                let output = Command::new("rustc")
                    .arg(file_path)
                    .output()?;

                if !output.status.success() {
                    println!("Failed to compile:");
                    io::stderr().write_all(&output.stderr)?;
                    break;
                }

                // Run the compiled binary
                Command::new(exec_path.clone())
                    .spawn()?
                    .wait()?;
                break;
            } else {
                code.push_str(&line);
            }
            line.clear();
        }

        // If user double enters then they exit the program otherwise we simply allow them 
        // to keep going with a clean file.
        println!("Do you wish to add more(continue) or finish(end)? If neither then press any key.");
        io::stdin().read_line(&mut line)?;
        if line.trim().to_lowercase() == "end" {
            // Remove binary and temp file
            Command::new("rm")
                .arg(file_path)
                .output()?;

            Command::new("rm")
                .arg(exec_path)
                .output()?;
            break;
        } else if line.trim().to_lowercase() == "continue" {
            continue;
        } else {
            code.clear();
            code.push_str("fn main() {");
        }
    }
    Ok(())
}

