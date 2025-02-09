use openssl::symm::{encrypt, decrypt, Cipher};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Input and output file paths
    let input_file = "letter_to_grandma.txt";
    let encrypted_file = "message.enc";
    let decrypted_file = "decrypted_letter.txt";

    // Password (in a real scenario, this should be securely handled)
    let password = "i didnt want to use virtual box, so i did it my own way :)";

    // Encrypt the file
    encrypt_file(input_file, encrypted_file, password)?;
    println!("File encrypted successfully: {}", encrypted_file);

    // Decrypt the file
    decrypt_file(encrypted_file, decrypted_file, password)?;
    println!("File decrypted successfully: {}", decrypted_file);

    Ok(())
}

fn encrypt_file(input_path: &str, output_path: &str, password: &str) -> io::Result<()> {
    // Read the input file
    let data = fs::read(input_path)?;

    // Generate a key and IV from the password using a simple key derivation function
    let (key, iv) = derive_key_and_iv(password);

    // Encrypt the data using AES-256-CBC
    let cipher = Cipher::aes_256_cbc();
    let encrypted_data = encrypt(cipher, &key, Some(&iv), &data).unwrap();

    // Write the encrypted data to the output file
    fs::write(output_path, encrypted_data)?;

    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str, password: &str) -> io::Result<()> {
    // Read the encrypted file
    let encrypted_data = fs::read(input_path)?;

    // Generate a key and IV from the password using the same key derivation function
    let (key, iv) = derive_key_and_iv(password);

    // Decrypt the data using AES-256-CBC
    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, &key, Some(&iv), &encrypted_data).unwrap();

    // Write the decrypted data to the output file
    fs::write(output_path, decrypted_data)?;

    Ok(())
}

fn derive_key_and_iv(password: &str) -> (Vec<u8>, Vec<u8>) {
    // In a real scenario, use a proper key derivation function like PBKDF2 or scrypt
    // For simplicity, we'll just pad the password to 32 bytes (256 bits) for the key
    // and 16 bytes (128 bits) for the IV.
    let mut key = password.as_bytes().to_vec();
    key.resize(32, 0); // Pad to 32 bytes (256 bits)

    let mut iv = password.as_bytes().to_vec();
    iv.resize(16, 0); // Pad to 16 bytes (128 bits)

    (key, iv)
}