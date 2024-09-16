# 🔒 Encryptor-LOL
Encryptor-LOL is a simple, fun command-line tool written in Rust that encrypts your messages using three different encryption methods: Caesar Cipher, XOR Cipher, and AES-128 (CBC Mode). 🔐✨

### YouTube Tutorial:

https://www.youtube.com/watch?v=xCkovNOh1VA
[![Watch the video](https://img.youtube.com/vi/xCkovNOh1VA/maxresdefault.jpg)](https://www.youtube.com/watch?v=xCkovNOh1VA)


### ⚡ Features

-🏛 **Caesar Cipher**: Shifts each letter of your message by a number of positions in the alphabet.

-🔑 **XOR Cipher**: Encrypts your message using an XOR operation with a key.

-🛡 **AES-128 Encryption**: Uses the AES-128 algorithm with Cipher Block Chaining (CBC) mode for secure encryption.

-🎨 **Colored Output**: Displays a cool ASCII art logo in blue at the start using the colored crate.

### 🎯 How It Works

1. 📝 The tool prompts you to enter a message.
2. It encrypts the message using:

   **-Caesar Cipher** with a default shift of 3.

   **-XOR Cipher** with a default key of 42.

   **-AES-128 Encryption** using a predefined 128-bit key and initialization vector (IV).
4. 🔐 Outputs the encrypted message for each method in the terminal.

### 🚀 Getting Started
Prerequisites

    Make sure you have installed on your system.

Installation

    Clone the repository to your local machine:

bash

    git clone https://github.com/miskoplisko/encryptor-lol.git

Navigate to the project folder:

bash

    cd encryptor-lol

Build the project with Cargo:

bash

    cargo build

Running the Tool

To run the program, simply execute:

bash

    cargo run 

After running, you'll see the following cool ASCII art in blue and a prompt to enter your message:

text

       _____                             _                  _     ___  _     
      | ____|_ __   ___ _ __ _   _ _ __ | |_ ___  _ __     | |   / _ \| |    
      |  _| | '_ \ / __| '__| | | | '_ \| __/ _ \| '__|____| |  | | | | |    
      | |___| | | | (__| |  | |_| | |_) | || (_) | | |_____| |__| |_| | |___ 
      |_____|_| |_|\___|_|   \__, | .__/ \__\___/|_|       |_____\___/|_____|
                             |___/|_|                                         
    [+] Tool that automatically ecrypt message, Written in Rust [+]
    [+] Subscribe to my channel youtube.com/@YTsight
    [+] HAVE FUN WITH MY TOOL :) 
    Enter the message you want to encrypt:

Example Output

Once you enter a message, the tool will encrypt it using the three methods and print the results:

bash

    Enter the message you want to encrypt:
    Hello, Rust!

    Original plaintext: Hello, Rust!
    Caesar Encrypted (shift 3): Khoor, Uxvw!
    XOR Encrypted (key 42):  ...
    AES Encrypted (Base64): ...

### 📂 Project Structure

    src/main.rs: The main program that handles user input and encryption logic.
    Cargo.toml: Contains the project's dependencies like colored, aes, base64, block-modes, and generic-array.

### 📦 Dependencies

    🔐 aes: For AES encryption.
    🧬 base64: To encode the AES-encrypted message in Base64 format.
    🔗 block-modes: For Cipher Block Chaining (CBC) mode encryption.
    🎨 colored: For displaying colored ASCII art in the terminal.
    🧮 generic-array: For managing fixed-size arrays used in encryption.

### 💡 Contributing

Contributions are welcome! Feel free to open issues for suggestions or create pull requests to improve the project.

### 📄 License

This project is licensed under the Apache-2.0 License. Check out the LICENSE file for details.

-------------------------------------------------------------------------------------------

Made with 💖 in Rust!
