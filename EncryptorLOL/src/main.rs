use aes::Aes128;
use base64::{encode as base64_encode};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use generic_array::GenericArray; // Import GenericArray
use std::io::{self}; // Import for handling user input
use colored::*; // Import for colored output
use std::str;

// AES encryption using CBC mode
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    // Display ASCII art text in blue
    let ascii_art = r#"
  _____                             _                  _     ___  _     
 | ____|_ __   ___ _ __ _   _ _ __ | |_ ___  _ __     | |   / _ \| |    
 |  _| | '_ \ / __| '__| | | | '_ \| __/ _ \| '__|____| |  | | | | |    
 | |___| | | | (__| |  | |_| | |_) | || (_) | | |_____| |__| |_| | |___ 
 |_____|_| |_|\___|_|   \__, | .__/ \__\___/|_|       |_____\___/|_____|
                        |___/|_|                                         

[+] Tool that automatically ecrypt message, Written in Rust [+]
[+] Subscribe to my channel youtube.com/@YTsight
[+] HAVE FUN WITH MY TOOL :) 
"#;

    println!("{}", ascii_art.blue());
    // Prompt the user to enter a message
    println!("Enter the message you want to encrypt:");

    // Create a mutable string to store user input
    let mut plaintext = String::new();
    
    // Read user input
    io::stdin().read_line(&mut plaintext).expect("Failed to read input");
    
    // Remove the newline from the input
    let plaintext = plaintext.trim();

    let caesar_shift = 3;
    let xor_key = 42;
    let aes_key = b"anexamplekey1234"; // Ensure AES key is 128-bit (16 bytes)
    let aes_iv = b"exampleiv1234567";  // Ensure AES IV is 128-bit (16 bytes)

    println!("Original plaintext: {}", plaintext);

    // Caesar cipher encryption
    let caesar_encrypted = caesar_encrypt(plaintext, caesar_shift);
    println!("Caesar Encrypted (shift {}): {}", caesar_shift, caesar_encrypted);

    // XOR cipher encryption
    let xor_encrypted = xor_encrypt(plaintext.as_bytes(), xor_key);
    println!("XOR Encrypted (key {}): {}", xor_key, String::from_utf8_lossy(&xor_encrypted));

    // AES encryption
    let aes_encrypted = aes_encrypt(plaintext, aes_key, aes_iv);
    println!("AES Encrypted (Base64): {}", aes_encrypted);
}

/// AES encryption using AES-128 CBC mode
fn aes_encrypt(plaintext: &str, key: &[u8; 16], iv: &[u8; 16]) -> String {
    // Convert key and IV to GenericArray
    let key = GenericArray::from_slice(key);
    let iv = GenericArray::from_slice(iv);

    let cipher = Aes128Cbc::new_fix(key, iv);
    let ciphertext = cipher.encrypt_vec(plaintext.as_bytes());
    base64_encode(&ciphertext) // Return ciphertext as base64-encoded string
}

// Caesar cipher encryption
fn caesar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            if c.is_ascii_alphabetic() {
                ((c as u8 - base + shift) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

// XOR encryption
fn xor_encrypt(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}
