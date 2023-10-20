use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory
    let current_dir: std::path::PathBuf = std::env::current_dir()?;
    
    // Read the contents of the current directory
    let entries = fs::read_dir(&current_dir)?;

    // Collect the entries into a vector of strings
    let items: Vec<String> = entries
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|entry| entry.file_name().into_string().ok())
        })
        .collect();

    let mut selected_index = 0;

    loop {
        // Clear the terminal screen (Unix-like systems)
        let _ = Command::new("clear").status();

        // Print the list of files and directories with highlighting for the selected item
        for (index, item) in items.iter().enumerate() {
            if index == selected_index {
                println!("> {}", item);
            } else {
                println!("  {}", item);
            }
        }

        // Read user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        // Parse user input and handle commands as needed
        match input.trim() {
            "q" => break, // Quit the program
            "j" => {
                // Navigate down (increment selected_index)
                selected_index = (selected_index + 1).min(items.len() - 1);
            }
            "k" => {
                // Navigate up (decrement selected_index)
                selected_index = selected_index.saturating_sub(1);
            }
            "l" => {
                // Select the file (implement further actions here)
                if let Some(selected_item) = items.get(selected_index) {
                    println!("Selected: {}", selected_item);
                    // You can implement file editing or other actions here
                }
            }
            _ => {
                println!("Invalid input. Use 'j' to go down, 'k' to go up, 'l' to select a file, or 'q' to quit.");
            }
        }
    }

    Ok(())
}

