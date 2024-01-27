use crate::password_manager::{CredentialSet, MaskedCredentialSet};
use prettytable::{row, Cell, Row, Table};
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use arboard::Clipboard;
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

//TODO - am i using this, it makes no sense, needs fixing
pub fn prompt_for_number(prompttxt: &str) -> Option<usize> {
    let input = prompt(prompttxt);
    if input.trim().is_empty() {
        None
    } else {
        input.trim().parse().ok()
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

pub fn get_home_dir() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

//TODO - Give this a return type?
pub fn copy_to_clipboard_then_clear(text: &str) {
    match Clipboard::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_text(text.to_owned()) {
                eprintln!("[ERROR]: Failed to copy to clipboard: {e}");
                return;
            }

            println!("[INFO]: Sensitive data copied to clipboard. It will be cleared in 20 seconds.");

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
