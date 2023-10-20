use termios::*;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::process::Command;
use std::io::Write;

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
        show_cursor();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // This will set the terminal to raw mode
    let _raw_terminal = RawTerminal::new()?;
    hide_cursor();

    // Get the current directory
    let current_dir = std::env::current_dir()?;
    // Read the contents of the current directory
    let entries = fs::read_dir(&current_dir)?;
    // Collect the entries into a vector of strings
    let mut items: Vec<String> = entries
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
            'a' => {
                print!("Add file: ");
                io::stdout().flush().unwrap();
                
                // Read the file name from the user
                let mut file_name = String::new();
                io::stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim();
                
                // Create the file in the current directory
                let path = current_dir.join(file_name);
                fs::File::create(&path).unwrap();
                
                // Refresh the list of files
                let entries = fs::read_dir(&current_dir)?;
                items = entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                    })
                    .collect();
                }
            _ => {}
        }
    }

    // No need to explicitly reset terminal settings here, as it will
    // be done automatically when _raw_terminal goes out of scope.
    Ok(())
}

fn hide_cursor() {
    print!("\x1B[?25l"); // This is the escape code to hide the cursor
    io::stdout().flush().unwrap(); // Ensure the print! output is flushed
}

fn show_cursor() {
    print!("\x1B[?25h"); // This is the escape code to show the cursor
    io::stdout().flush().unwrap(); // Ensure the print! output is flushed
}

