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

### Getting Started (Unix)
To get started with ArmorPass on Unix systems, follow these steps:

1. **Install OpenSSL**:
   - Ensure OpenSSL is installed on your system. You can install it using your package manager. For example, on Ubuntu/Debian:
     ```
     sudo apt-get install openssl libssl-dev
     ```

2. **Clone the Repository**:
   - Clone the ArmorPass repository:
     ```
     git clone https://github.com/yourusername/ArmorPass.git
     ```

3. **Build the Project**:
   - Navigate to the ArmorPass directory and build the project using Cargo (Rust's package manager):
     ```
     cd ArmorPass
     cargo build
     ```

4. **Run ArmorPass**:
   - After building, you can run the application:
     ```
     cargo run
     ```

5. **Using ArmorPass**:
   - Use the command-line interface to manage your passwords. For example, to create a new set of credentials, run `create`.

### Getting Started (Windows)
Setting up ArmorPass on Windows involves a few additional steps to handle OpenSSL dependencies:

1. **Install OpenSSL using vcpkg**:
   - Install [vcpkg](https://github.com/microsoft/vcpkg) from Microsoft.
   - Install OpenSSL via vcpkg:
     ```
     .\vcpkg\vcpkg install openssl:x64-windows
     ```

2. **Set Environment Variables**:
   - Set the required environment variables so that Cargo can locate the OpenSSL libraries. Open PowerShell and run:
     ```
     $Env:OPENSSL_DIR = "C:\path\to\vcpkg\installed\x64-windows"
     $Env:OPENSSL_LIB_DIR = "C:\path\to\vcpkg\installed\x64-windows\lib"
     $Env:OPENSSL_INCLUDE_DIR = "C:\path\to\vcpkg\installed\x64-windows\include"
     ```
   - Replace `C:\path\to\vcpkg` with your actual vcpkg installation path.

3. **Clone, Build, and Run (as with Unix)**:
   - Follow the same steps as for Unix users to clone, build, and run ArmorPass.

**Note**: If you encounter a `STATUS_DLL_NOT_FOUND` error, ensure that the OpenSSL DLLs are in your system PATH or in the same directory as the ArmorPass executable.

### Troubleshooting
- If you run into any issues during installation or running ArmorPass, please check the project's [issue tracker](https://github.com/yourusername/ArmorPass/issues) or open a new issue detailing the problem.

### Contributing
- Contributions, bug reports, and suggestions are welcome! Please see the [Contributing](#Contributing) section for more details.


## Planned Features
- **Offer Statically Linked Binary**: Update the build pipeline to produce standalone binaries for ease of use.
- **Sync Between Devices**: Implement a secure method to synchronize the password file across multiple devices, ensuring seamless access to credentials from any location.
- **Two-Factor Authentication (2FA)**: Add support for two-factor authentication for added security during the login process.
- **Auto-Fill Functionality**: Develop a browser extension or integration to auto-fill passwords on web pages, enhancing user convenience.

## Contributing
Contributions to ArmorPass are welcome! If you have suggestions, bug reports, or contributions, please open an issue or a pull request.

## Disclaimer
ArmorPass is a project developed for educational purposes. While it aims to implement robust security practices, as a learning project, it may not meet all the security standards required for a production-grade password manager.

