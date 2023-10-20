use termios::*;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::process::Command;

fn get_terminal_settings() -> Result<Termios, std::io::Error> {
    let fd = 0; // This is the file descriptor for stdin
    Termios::from_fd(fd)
}

fn set_raw_mode(original_settings: &Termios) -> Result<(), std::io::Error> {
    let fd = 0; // This is the file descriptor for stdin
    let mut raw = original_settings.clone();
    raw.c_lflag &= !(ECHO | ICANON);
    tcsetattr(fd, TCSANOW, &raw)
}

fn reset_terminal_settings(original_settings: &Termios) -> Result<(), std::io::Error> {
    let fd = 0; // This is the file descriptor for stdin
    tcsetattr(fd, TCSANOW, original_settings)
}

fn read_single_key() -> io::Result<char> {
    let mut buf = [0];
    io::stdin().read_exact(&mut buf)?;
    Ok(buf[0] as char)
}

fn main() -> Result<(), Box<dyn Error>> {
    let original_settings = get_terminal_settings()?;
    set_raw_mode(&original_settings)?;

    // Get the current directory
    let current_dir = std::env::current_dir()?;
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
        match read_single_key()? {
            'q' => break, // Quit the program
            'j' => {
                // Navigate down (increment selected_index)
                selected_index = (selected_index + 1).min(items.len() - 1);
            }
            'k' => {
                // Navigate up (decrement selected_index)
                selected_index = selected_index.saturating_sub(1);
            }
            'l' => {
                // Select the file (implement further actions here)
                if let Some(selected_item) = items.get(selected_index) {
                    println!("Selected: {}", selected_item);
                    // You can implement file editing or other actions here
                }
            }
            _ => {}
        }
    }

    // Reset terminal settings before exiting
    reset_terminal_settings(&original_settings)?;

    Ok(())
}

// Todo: use this:
struct RawTerminal {
    original_settings: Termios,
}

impl RawTerminal {
    fn new() -> Result<Self, std::io::Error> {
        let original_settings = get_terminal_settings()?;
        set_raw_mode(&original_settings)?;
        Ok(RawTerminal { original_settings })
    }
}

impl Drop for RawTerminal {
    fn drop(&mut self) {
        // We intentionally don't care if this call fails here because
        // this is our last-ditch effort to reset the terminal.
        let _ = reset_terminal_settings(&self.original_settings);
    }
}

