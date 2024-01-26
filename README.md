# ArmorPass Password Manager

## Overview
ArmorPass is a password management application developed as a learning project to deepen understanding of Rust and encryption techniques. The application is currently suitable for Unix systems with plans to port it to Windows in the near future. 

ArmorPass is designed with security in mind. It employs state-of-the-art encryption standards to ensure your credentials are securely stored. Each write operation to the data file involves re-encryption, enhancing data integrity and preventing unauthorized access.

### Goals
- Serve as a Rust learning project
- Gain practical experience with encryption methodologies

### Key Features
- **Encryption**: Utilizes OpenSSL, a robust C library, interfaced through Rust's foreign function interface, ensuring high-performance cryptographic operations.
- **Key Derivation**: Employs `pbkdf2_hmac` as a key derivation function, fortifying the security by transforming the input password into a cryptographically strong key.
- **Encryption Algorithm**: Implements `aes_256_cbc`, an industry-standard encryption algorithm known for its strength and reliability.

### Commands
ArmorPass offers a straightforward and intuitive command-line interface with the following commands:

- `create`: Initialize a new set of credentials.
- `delete`: Remove an existing set of credentials.
- `retrieve`: Fetch and display credentials for a specific identifier.
- `retrieveall`: Retrieve and list all credentials associated with a particular identifier.
- `update`: Update existing credentials.
- `quit` or `exit`: Close the application.

### Multiple Usernames per Identifier
ArmorPass allows you to associate multiple usernames with a single identifier (e.g., `abc.com` can have `abc1`, `abc2`, `abc3`). This feature is particularly useful for managing different accounts on the same platform or service. Using the `retrieveall` command, you can prompt for an identifier and the application will list credentials for all username entries associated with that identifier.

## Getting Started - TODO

## Planned Features
- **Windows Support**: Upcoming port to Windows to make ArmorPass a cross-platform solution for password management.
- **Enhanced Security Features**: Continuous improvement of security features to ensure your credentials are protected with the most advanced technologies.

## Contributing
Contributions to ArmorPass are welcome! If you have suggestions, bug reports, or contributions, please open an issue or a pull request.

## Disclaimer
ArmorPass is a project developed for educational purposes. While it aims to implement robust security practices, as a learning project, it may not meet all the security standards required for a production-grade password manager.

