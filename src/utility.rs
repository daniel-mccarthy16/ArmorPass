use crate::password_manager::{CredentialSet, MaskedCredentialSet};
use arboard::Clipboard;
use prettytable::{row, Cell, Row, Table};
use std::env;
use std::error::Error;
use std::fs;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::{thread, time::Duration};

pub fn validate_identifier(identifier: &str) -> Result<(), ArmorPassError> {
    if !is_at_least_three_characters_long(identifier) {
        Err(ArmorPassError::CreateIdentifierTooShort)
    } else {
        Ok(())
    }
}

fn is_at_least_three_characters_long(password: &str) -> bool {
    password.len() >= 3
}

pub fn prompt(prompttext: &str) -> String {
    print!(">> {}", prompttext);
    stdout().flush().unwrap(); // TODO - dangerous or not?

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

pub fn prompt_for_number(prompttxt: &str) -> Option<u8> {
    loop {
        let input = prompt(prompttxt);
        if input.trim().is_empty() {
            return None;
        } else {
            match input.trim().parse::<u8>() {
                Ok(num) => return Some(num),
                Err(_) => {
                    eprintln!("Please enter a valid number (0-255)");
                    continue;
                }
            }
        }
    }
}

pub fn prompt_for_confirmation(prompttxt: &str) -> bool {
    let input = prompt(prompttxt).trim().to_lowercase();
    matches!(input.as_str(), "y" | "yes")
}

pub fn print_credential_list(credential_list: Vec<MaskedCredentialSet>) {
    let mut table = Table::new();
    table.add_row(row!["Identifier", "Username", "Password"]);
    for credential in credential_list {
        table.add_row(Row::new(vec![
            Cell::new(&credential.identifier),
            Cell::new(&credential.username),
            Cell::new(&credential.password),
        ]));
    }
    table.printstd();
}

pub fn print_credential(credential: &CredentialSet) {
    let mut table = Table::new();
    table.add_row(row!["Identifier", "Username", "Password"]);
    table.add_row(Row::new(vec![
        Cell::new(&credential.identifier),
        Cell::new(&credential.username),
        Cell::new(&credential.password),
    ]));
    table.printstd();
}

pub fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE")
    } else {
        env::var("HOME")
    };
    match home_dir {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(e) => Err(Box::new(e)),
    }
}

//TODO - Give this a return type?
pub fn copy_to_clipboard_then_clear(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text.to_owned()) {
                eprintln!("[ERROR]: Failed to copy to clipboard: {e}");
                return;
            }

            println!(
                "[INFO]: Sensitive data copied to clipboard. It will be cleared in 20 seconds."
            );

            let content_to_clear = text.to_owned();

            // Spawn a new thread to clear the clipboard after 20 seconds
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(20));
                match clipboard.get_text() {
                    Ok(current_content) if current_content == content_to_clear => {
                        let _ = clipboard.set_text("".to_owned());
                    }
                    Ok(_) => {} // The content has changed, do not clear the clipboard.
                    Err(e) => eprintln!("[ERROR]: Failed to get clipboard content: {}", e),
                }
            });
        }
        Err(e) => eprintln!("[ERROR]: Failed to instantiate clipboard: {e}"),
    }
}

pub fn armor_file_exists() -> bool {
    let home_dir = get_home_dir().unwrap();
    let file_path = home_dir.join(".armorpass.enc");
    match fs::metadata(file_path) {
        Ok(_) => true,
        Err(_e) => false,
    }
}

#[derive(Debug, PartialEq)]
pub enum ArmorPassError {
    CreateDuplicateUsername,
    CreateDuplicatePassword,
    CreateIdentifierTooShort,
    FailedToPersistToDisk(String),
    NoRecordFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_identifier() {
        assert_eq!(validate_identifier("id123"), Ok(()));
        assert!(
            matches!(
                validate_identifier("id"),
                Err(ArmorPassError::CreateIdentifierTooShort)
            ),
            "Expected CreateIdentifierTooShort error"
        );
    }
}
