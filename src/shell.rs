use crate::generator::PasswordGenerator;
use crate::generator::PasswordGeneratorOptions;
use crate::password_manager::PasswordManager;
use crate::strings::{PROMPT_MAIN_COMMAND, PROMPT_MASTER_PASSWORD};
use crate::utility::armor_file_exists;
use crate::utility::copy_to_clipboard_then_clear;
use crate::utility::get_home_dir;
use crate::utility::print_credential_list;
use crate::utility::prompt;

enum Command {
    Create(CreatePasswordOptions),
    Delete(DeletePasswordOptions),
    Retrieve(RetrieveSingleOptions),
    RetrieveAll(RetrieveAllOptions),
    Update(UpdatePasswordOptions),
    Quit,
}

#[derive(Default)]
pub struct CreatePasswordOptions {
    pub identifier: String,
    pub username: String,
    pub password: String,
}

#[derive(Default)]
pub struct UpdatePasswordOptions {
    pub identifier: String,
    pub username: String,
    pub password: String,
}

#[derive(Default)]
pub struct RetrieveSingleOptions {
    pub identifier: String,
    pub username: String,
}

#[derive(Default)]
pub struct RetrieveAllOptions {
    pub identifier: String,
}

#[derive(Default)]
pub struct DeletePasswordOptions {
    pub identifier: String,
    pub username: String,
}

impl Command {
    fn from_str(command_str: &str) -> Option<Command> {
        match command_str {
            cs if cs.eq_ignore_ascii_case("create") => {
                Some(Command::Create(CreatePasswordOptions::default()))
            }
            cs if cs.eq_ignore_ascii_case("delete") => {
                Some(Command::Delete(DeletePasswordOptions::default()))
            }
            cs if cs.eq_ignore_ascii_case("retrieve") => {
                Some(Command::Retrieve(RetrieveSingleOptions::default()))
            }
            cs if cs.eq_ignore_ascii_case("retrieveall") => {
                Some(Command::RetrieveAll(RetrieveAllOptions::default()))
            }
            cs if cs.eq_ignore_ascii_case("update") => {
                Some(Command::Update(UpdatePasswordOptions::default()))
            }
            cs if cs.eq_ignore_ascii_case("quit")
                || cs.eq_ignore_ascii_case("exit")
                || cs.eq_ignore_ascii_case("q") =>
            {
                Some(Command::Quit)
            }
            _ => None,
        }
    }

    fn execute(&mut self, shell: &mut Shell) {
        match self {
            Command::Create(options) => shell.handle_create_command(options),
            Command::Delete(options) => shell.handle_delete_command(options),
            Command::Retrieve(options) => shell.handle_retrieve_command(options),
            Command::RetrieveAll(options) => shell.handle_retrieve_all_command(options),
            Command::Update(options) => shell.handle_update_command(options),
            Command::Quit => shell.should_terminate = true,
        }
    }
}

enum ShellState {
    Main,
    Authenticate,
    Initialization,
}

pub struct Shell {
    state: ShellState,
    should_terminate: bool,
    password_manager: Option<PasswordManager>,
}

impl Default for Shell {
    fn default() -> Shell {
        Shell {
            should_terminate: false,
            state: ShellState::Authenticate,
            password_manager: None,
        }
    }
}

impl Shell {
    pub fn new() -> Shell {
        let initial_state = if armor_file_exists() {
            ShellState::Authenticate
        } else {
            ShellState::Initialization
        };
        Shell {
            should_terminate: false,
            state: initial_state,
            password_manager: None,
        }
    }

    pub fn run(&mut self) {
        while !self.should_terminate {
            match self.state {
                ShellState::Main => {
                    let input = prompt(PROMPT_MAIN_COMMAND);
                    self.handle_main_command(&input);
                }
                ShellState::Authenticate => {
                    let masterpassword = prompt(PROMPT_MASTER_PASSWORD);
                    self.handle_authentication_prompt(&masterpassword);
                }
                ShellState::Initialization => {
                    self.handle_initialization();
                }
            }
        }
    }

    fn handle_main_command(&mut self, input: &str) {
        if let Some(mut command) = Command::from_str(input) {
            command.execute(self);
        } else {
            self.show_root_prompt_help_message();
        }
    }

    fn show_root_prompt_help_message(&self) {
        println!("Welcome to the armor pass shell! Here are the available commands:");
        println!("1. Create - Use this command to create a new item.");
        println!("2. Delete - Use this command to delete an existing item.");
        println!("3. Retrieve - Use this command to retrieve details of an existing item.");
        println!("4. RetrieveAll - Use this command to retrieve everything for an identifier");
        println!("5. Update - Use this command to update details of an existing item.");
        println!("6. Quit - Use this command to exit the application.");
        println!("\nType a command and press Enter to execute it.");
    }

    fn handle_authentication_prompt(&mut self, masterpassword: &str) {
        let home_dir = get_home_dir()
            .expect("[ERROR]: could not find home directory, is HOME env variable missing?");
        let file_path = home_dir.join(".armorpass.enc");
        match PasswordManager::new(file_path, masterpassword) {
            Ok(password_manager) => {
                self.state = ShellState::Main;
                self.password_manager = Some(password_manager);
            }
            Err(e) => {
                eprintln!("Failed auth attempt: {}", e);
            }
        }
    }

    fn handle_initialization(&mut self) {
        println!("Welcome to the ArmorPass setup wizard!");
        let mut input;
        let mut input2;
        loop {
            input = prompt("Please set your password");
            input2 = prompt("Please re-enter your password for confirmation");
            if input == input2 {
                break;
            }
        }
        let home_dir = get_home_dir()
            .expect("[ERROR]: could not find home directory, is HOME env variable missing?");
        let file_path = home_dir.join(".armorpass.enc");
        match PasswordManager::new(file_path, &input) {
            Ok(password_manager) => {
                self.state = ShellState::Main;
                self.password_manager = Some(password_manager);
            }
            Err(e) => {
                eprintln!("Failed auth attempt: {}", e);
            }
        }
    }

    fn handle_create_command(&mut self, options: &mut CreatePasswordOptions) {
        options.identifier = self.prompt_for_identifier();
        options.username = self.prompt_for_username();

        let mut password_generator_options = PasswordGeneratorOptions::default();
        password_generator_options.prompt_for_options();
        let password_generator = PasswordGenerator::new(&password_generator_options);
        options.password = password_generator.generate();

        let password_manager = self.get_password_manager_mut();
        let _ = password_manager.store_password(options);
    }

    fn handle_delete_command(&mut self, options: &mut DeletePasswordOptions) {
        options.identifier = self.prompt_for_identifier();
        options.username = self.prompt_for_username();

        let password_manager = self.get_password_manager_mut();

        match password_manager.delete_credential(options) {
            Ok(_) => {
                println!(
                    "successfully deleted credential with identifer: {} and username: {}",
                    &options.identifier, &options.username
                );
            }
            Err(_e) => (),
        }
    }

    fn handle_retrieve_all_command(&mut self, options: &mut RetrieveAllOptions) {
        options.identifier = self.prompt_for_identifier();
        let password_manager = self.get_password_manager_mut();
        let credential_list = password_manager.retrieve_all_credentials_masked(options);
        if credential_list.is_empty() {
            eprintln!("[Warn]: Could not find any records for that identifier");
        } else {
            print_credential_list(credential_list);
        }
    }

    fn handle_retrieve_command(&mut self, options: &mut RetrieveSingleOptions) {
        options.identifier = self.prompt_for_identifier();
        options.username = self.prompt_for_username();
        let password_manager = self.get_password_manager_mut();
        match password_manager.retrieve_credential(options) {
            Some(credential) => {
                copy_to_clipboard_then_clear(&credential.password);
            }
            None => eprintln!(
                "[Warn]: Could not find a record for that identifier/username combination"
            ),
        }
    }

    fn handle_update_command(&mut self, options: &mut UpdatePasswordOptions) {
        options.identifier = self.prompt_for_identifier();
        options.username = self.prompt_for_username();

        let mut password_generator_options = PasswordGeneratorOptions::default();
        password_generator_options.prompt_for_options();
        let password_generator = PasswordGenerator::new(&password_generator_options);
        options.password = password_generator.generate();

        let password_manager = self.get_password_manager_mut();

        if password_manager.update_password(options).is_ok() {
            println!(
                "succesfully updated password for identifier: {} with username: {}",
                options.identifier.as_str(),
                options.username.as_str()
            )
        }
    }

    fn get_password_manager_mut(&mut self) -> &mut PasswordManager {
        self.password_manager
            .as_mut()
            .expect("[ERROR]: havent yet unencrypted file for operation, authentication required")
    }

    fn prompt_for_identifier(&mut self) -> String {
        let mut identifer: String = String::new();
        while identifer.is_empty() {
            identifer = prompt("Enter an identifier [cannot be empty]: ");
        }
        identifer
    }

    fn prompt_for_username(&mut self) -> String {
        prompt("Enter a username: ")
    }
}
