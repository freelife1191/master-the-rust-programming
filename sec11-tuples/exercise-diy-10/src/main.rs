fn execute_command_original(mode: &str, status: &str) {
    if mode == "admin" && status == "active" {
        println!("Admin privileges granted. Executing active command.");
    } else if (mode == "normal" && status == "pending") || (mode == "normal" && status == "active")
    {
        println!("Normal operation. Execute pending or active commands.");
    } else if mode == "maintenance" && status == "complete" {
        println!("System maintenance complete. Ready for normal operation.");
    } else {
        println!("No action needed or invalid mode/status.");
    }
}

//Write the refactored code using tuple and pattern matching using 'match' statement
fn execute_command_refactored(mode: &str, status: &str) {
    match (mode, status) {
        ("admin", "active") => {
            println!("Admin privileges granted. Executing active command.");
        }

        ("normal", "pending") | ("normal", "active") => {
            println!("Normal operation. Execute pending or active commands.");
        }

        ("maintenance", "complete") => {
            println!("System maintenance complete. Ready for normal operation.");
        }

        _ =>  println!("No action needed or invalid mode/status."),
    }
}

fn main() {
    execute_command_refactored("admin", "active");
    execute_command_refactored("normal", "pending");
    execute_command_refactored("maintenance", "complete");
    execute_command_refactored("admin", "pending");
}
