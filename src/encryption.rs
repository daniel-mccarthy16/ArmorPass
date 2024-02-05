use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::rand::rand_bytes;
use openssl::symm::{Cipher, Crypter, Mode};
use std::fs::{File, Permissions};
use std::io::{Read, Write};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const ITERATIONS: usize = 100_000;
const KEY_LENGTH: usize = 32; //32 bytes = 256bit which is the key length size aes_256_cbc expects
const IV_LENGTH: usize = 16;
const SALT_LENGTH: usize = 16;

pub struct CryptoManager {
    salt: Vec<u8>,
    iv: Vec<u8>,
    ciphertext: Vec<u8>,
    key: Vec<u8>,
    filepath: PathBuf,
}

impl CryptoManager {
    pub fn new(filepath: &PathBuf, password: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match File::open(filepath) {
            Ok(mut file) => {
                let mut salt = vec![0u8; SALT_LENGTH];
                file.read_exact(&mut salt)?;

                let mut iv = vec![0u8; IV_LENGTH];
                file.read_exact(&mut iv)?;

                let mut ciphertext = Vec::new();
                file.read_to_end(&mut ciphertext)?;

                let key = CryptoManager::generate_key(password, &salt)?;

                Ok(CryptoManager {
                    salt,
                    iv,
                    ciphertext,
                    key,
                    filepath: filepath.clone(),
                })
            }
            Err(_) => {
                let salt = CryptoManager::generate_salt(SALT_LENGTH)?;
                let iv = CryptoManager::generate_iv(IV_LENGTH)?;
                let key = CryptoManager::generate_key(password, &salt)?;

                Ok(CryptoManager {
                    salt,
                    iv,
                    ciphertext: Vec::new(),
                    key,
                    filepath: filepath.clone(),
                })
            }
        }
    }

    pub fn encrypt_and_persist(&mut self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_data = self.encrypt_data(data)?;
        self.ciphertext = encrypted_data;
        let mut file = File::create(&self.filepath)?;
        file.write_all(&self.salt)?;
        file.write_all(&self.iv)?;
        file.write_all(&self.ciphertext)?;
        #[cfg(unix)]
        {
            let permissions = Permissions::from_mode(0o600);
            std::fs::set_permissions(&self.filepath, permissions)?;
        }
        Ok(())
    }

    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, &self.key, Some(&self.iv))?;
        crypter.pad(true);
        let mut encrypted = vec![0; data.len() + cipher.block_size()];
        let count = crypter.update(data, &mut encrypted)?;
        let rest = crypter.finalize(&mut encrypted[count..])?;
        encrypted.truncate(count + rest);
        Ok(encrypted)
    }

    pub fn decrypt_and_retrieve(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.ciphertext.is_empty() {
            return Ok(Vec::new());
        }
        let decrypted_data = self.decrypt_data(&self.ciphertext)?;
        Ok(decrypted_data)
    }

    fn decrypt_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let cipher = Cipher::aes_256_cbc();
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, &self.key, Some(&self.iv))?;
        crypter.pad(true);
        let mut decrypted = vec![0; encrypted_data.len() + cipher.block_size()];
        let count = crypter.update(encrypted_data, &mut decrypted)?;
        let rest = crypter.finalize(&mut decrypted[count..])?;
        decrypted.truncate(count + rest);
        Ok(decrypted)
    }

    fn generate_salt(length: usize) -> Result<Vec<u8>, ErrorStack> {
        let mut buffer = vec![0u8; length];
        rand_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn generate_iv(length: usize) -> Result<Vec<u8>, ErrorStack> {
        let mut buffer = vec![0u8; length];
        rand_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn generate_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, ErrorStack> {
        let password_bytes = password.as_bytes();
        let mut key = vec![0u8; KEY_LENGTH];
        pbkdf2_hmac(
            password_bytes,
            salt,
            ITERATIONS,
            MessageDigest::sha256(),
            &mut key,
        )?;
        Ok(key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use uuid::Uuid;

    const TEST_PASSWORD: &str = "test_password";

    fn teardown(filepath: &PathBuf) {
        let _ = std::fs::remove_file(filepath);
    }

    fn generate_unique_file_path() -> PathBuf {
        let unique_id = Uuid::new_v4().to_string();
        PathBuf::from(format!("/tmp/test_{}.enc", unique_id))
    }

    #[test]
    fn test_generate_key() {
        let salt = CryptoManager::generate_salt(SALT_LENGTH).unwrap();
        let key = CryptoManager::generate_key(TEST_PASSWORD, &salt).unwrap();
        assert_eq!(key.len(), KEY_LENGTH);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let testfilepath = generate_unique_file_path();
        let salt = CryptoManager::generate_salt(SALT_LENGTH).unwrap();
        let iv = CryptoManager::generate_iv(IV_LENGTH).unwrap();
        let key = CryptoManager::generate_key(TEST_PASSWORD, &salt).unwrap();

        let crypto_manager = CryptoManager {
            salt,
            iv,
            ciphertext: Vec::new(),
            key,
            filepath: testfilepath.clone(),
        };

        let data = b"Hello, world!";
        let encrypted_data = crypto_manager.encrypt_data(data).unwrap();
        let decrypted_data = crypto_manager.decrypt_data(&encrypted_data).unwrap();

        assert_eq!(decrypted_data, data);
        teardown(&testfilepath);
    }

    #[test]
    fn test_generate_salt() {
        let salt = CryptoManager::generate_salt(SALT_LENGTH).unwrap();
        assert_eq!(salt.len(), SALT_LENGTH);
    }

    #[test]
    fn test_generate_iv() {
        let iv = CryptoManager::generate_iv(IV_LENGTH).unwrap();
        assert_eq!(iv.len(), IV_LENGTH);
    }

    #[test]
    fn test_new_instance_creation() {
        let testfilepath = generate_unique_file_path();
        let instance = CryptoManager::new(&testfilepath, TEST_PASSWORD);
        assert!(instance.is_ok());
        teardown(&testfilepath);
    }

    #[test]
    fn test_encrypt_and_persist_method() {
        let testfilepath = generate_unique_file_path();
        let mut crypto_manager = CryptoManager::new(&testfilepath, TEST_PASSWORD).unwrap();
        let data = b"Test data";
        let result = crypto_manager.encrypt_and_persist(data);
        assert!(result.is_ok());
        teardown(&testfilepath);
    }

    #[test]
    fn test_salt_stored_correctly_in_encrypted_file() {
        let testfilepath = generate_unique_file_path();
        let mut crypto_manager = CryptoManager::new(&testfilepath, TEST_PASSWORD).unwrap();
        let data = b"Test data";

        let _ = crypto_manager.encrypt_and_persist(data);

        let mut file = File::open(testfilepath.clone()).expect("Couldnt open file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Couldnt read file");

        let salt_from_file = &contents[0..SALT_LENGTH];

        assert_eq!(salt_from_file, crypto_manager.salt.as_slice());

        teardown(&testfilepath);
    }

    #[test]
    fn test_iv_stored_correctly_in_encrypted_file() {
        let testfilepath = generate_unique_file_path();
        let mut crypto_manager = CryptoManager::new(&testfilepath, TEST_PASSWORD).unwrap();
        let data = b"Test data";

        let _ = crypto_manager.encrypt_and_persist(data);

        let mut file = File::open(testfilepath.clone()).expect("Couldn't open fle");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .expect("Could not read file");

        let iv_from_file = &contents[SALT_LENGTH..(SALT_LENGTH + IV_LENGTH)];

        assert_eq!(iv_from_file, crypto_manager.iv.as_slice());

        teardown(&testfilepath);
    }
}

// ENCRYPTING
// 1. collect password
// 2. create random salt of fixed length ( if creating new file ) else use one on file itself
// 3. feed KDF the salt and pass to generate DERIVED KEY ( OF FIXED LENGTH )
// 4. feed encryption alg the IV and DERIVED KEY to produce CIPHERTEXT
// 5. write [SALT][IV][CIPHERTEXT] to file

// DECRYPTING
// 1. collect SALT/IV/CIPHERTEXT from file
// 2. use SALT / PASS to generate DERIVED KEY
// 3. use DERIVED KEY and IV to generate PLAINTEXT from CIPHERTEXT
//
// NOTES
// create salt only on file creation
