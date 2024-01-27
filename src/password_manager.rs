use crate::encryption::CryptoManager;
use crate::shell::CreatePasswordOptions;
use crate::shell::DeletePasswordOptions;
use crate::shell::RetrieveAllOptions;
use crate::shell::RetrieveSingleOptions;
use crate::shell::UpdatePasswordOptions;
use crate::utility::{validate_identifier, ArmorPassError};

use std::path::PathBuf;

pub struct PasswordManager {
    records: Vec<CredentialSet>,
    crypto_manager: CryptoManager,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CredentialSet {
    pub identifier: String,
    pub username: String,
    pub password: String,
}

impl CredentialSet {
   pub fn mask(&self) -> MaskedCredentialSet {
        MaskedCredentialSet {
            identifier: self.identifier.clone(),
            username: self.username.clone(),
            password: "*".repeat(self.password.len())
        }
   }
}

#[derive(Debug)]
pub struct MaskedCredentialSet {
    pub identifier: String,
    pub username: String,
    pub password: String
}

impl PasswordManager {
    pub fn new(
        armorpass_path: PathBuf,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let new_crypto_manager = CryptoManager::new(&armorpass_path, password)?;
        let stored_credentials = new_crypto_manager.decrypt_and_retrieve()?;
        let deserialized_records = if !stored_credentials.is_empty() {
            serde_json::from_slice(&stored_credentials)?
        } else {
            Vec::new()
        };
        Ok(PasswordManager {
            records: deserialized_records,
            crypto_manager: new_crypto_manager,
        })
    }

    pub fn store_password(
        &mut self,
        options: &CreatePasswordOptions,
    ) -> Result<(), ArmorPassError> {
        if self.password_is_duplicate(&options.password) {
            eprintln!("[ERROR]: Password must be unique");
            return Err(ArmorPassError::CreateDuplicatePassword);
        }

        if self.username_is_duplicate(&options.username) {
            eprintln!("[ERROR]: username must be unique");
            return Err(ArmorPassError::CreateDuplicateUsername);
        }

        validate_identifier(&options.identifier)?;

        let new_credentials = CredentialSet {
            identifier: options.identifier.to_string(),
            username: options.username.to_string(),
            password: options.password.to_string(),
        };

        self.records.push(new_credentials);

        Self::persist_credentials(self)?;

        Ok(())
    }

    pub fn has_password(&self, identifier: &str, username: &str) -> bool {
        self.records
            .iter()
            .any(|record| record.identifier == identifier && record.username == username)
    }

    pub fn retrieve_credential(&self, options: &RetrieveSingleOptions) -> Option<&CredentialSet> {
        if let Some(record) = self.records.iter().find(|&record| {
            record.identifier == options.identifier && record.username == options.username
        }) {
            Some(record)
        } else {
            None
        }
    }

    pub fn update_password(
        &mut self,
        options: &UpdatePasswordOptions,
    ) -> Result<(), ArmorPassError> {
        // Find a mutable reference to the record that needs updating.
        if let Some(record) = self.records.iter_mut().find(|record| {
            record.identifier == options.identifier && record.username == options.username
        }) {
            // If found, update the password field.
            record.password = options.password.to_string();
            Self::persist_credentials(self)?;
            Ok(())
        } else {
            Err(ArmorPassError::NoRecordFound)
        }
    }

    pub fn delete_credential(
        &mut self,
        options: &DeletePasswordOptions,
    ) -> Result<(), ArmorPassError> {
        // Store the original length to determine if a record was deleted.
        let original_len = self.records.len();

        // Retain only the records that do not match the identifier and username.
        self.records.retain(|record| {
            record.identifier != options.identifier || record.username != options.username
        });

        // Check if the records collection has changed in size.
        if self.records.len() == original_len {
            // No records were deleted, return an error.
            Err(ArmorPassError::NoRecordFound)
        } else {
            // A record was deleted, persist the changes.
            Self::persist_credentials(self)?;
            Ok(())
        }
    }

    pub fn retrieve_all_credentials(&self, options: &RetrieveAllOptions) -> Vec<&CredentialSet> {
        let identifer_name: &str = options.identifier.as_str();
        self.records
            .iter()
            .filter(|&record| record.identifier == identifer_name)
            .collect()
    }

    pub fn retrieve_all_credentials_masked(&self, options: &RetrieveAllOptions) -> Vec<MaskedCredentialSet> {
        let identifer_name: &str = options.identifier.as_str();
        self.records
            .iter()
            .filter(|&record| record.identifier == identifer_name)
            .map(|record| record.mask())
            .collect()
    }

    fn persist_credentials(&mut self) -> Result<(), ArmorPassError> {
        let json_data = serde_json::to_string(&self.records).map_err(|e| {
            ArmorPassError::FailedToPersistToDisk(format!(
                "Failed to serialize records to json: {}",
                e
            ))
        })?;

        self.crypto_manager
            .encrypt_and_persist(&json_data.into_bytes())
            .map_err(|e| {
                ArmorPassError::FailedToPersistToDisk(format!(
                    "Failed to encrypt and persist data: {}",
                    e
                ))
            })?;

        Ok(())
    }

    fn password_is_duplicate(&self, password: &str) -> bool {
        self.records
            .iter()
            .any(|record| record.password == password)
    }

    fn username_is_duplicate(&self, username: &str) -> bool {
        self.records
            .iter()
            .any(|record| record.username == username)
    }
}
