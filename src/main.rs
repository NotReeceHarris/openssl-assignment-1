use openssl::rsa::{Rsa, Padding};
use openssl::symm::{encrypt, decrypt, Cipher};
use rand::Rng;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Input and output file paths
    let input_file = "letter_to_grandma.txt";
    let encrypted_file = "message.enc";
    let decrypted_file = "decrypted_letter.txt";

    // Generate RSA key pair (in a real scenario, these would be securely stored)
    let rsa = Rsa::generate(2048).unwrap();
    let private_key = rsa.private_key_to_pem().unwrap();
    let public_key = rsa.public_key_to_pem().unwrap();

    // Generate a random symmetric key and IV for AES-256-CBC
    let (aes_key, aes_iv) = generate_aes_key_and_iv();

    // Encrypt the file using the symmetric key
    encrypt_file(input_file, encrypted_file, &aes_key, &aes_iv)?;
    println!("File encrypted successfully: {}", encrypted_file);

    // Encrypt the symmetric key using the public key
    let encrypted_aes_key = encrypt_aes_key_with_rsa(&aes_key, &public_key).unwrap();
    let encrypted_aes_iv = encrypt_aes_key_with_rsa(&aes_iv, &public_key).unwrap();

    // Save the encrypted symmetric key and IV to a file (for demonstration purposes)
    fs::write("encrypted_aes_key.bin", &encrypted_aes_key)?;
    fs::write("encrypted_aes_iv.bin", &encrypted_aes_iv)?;

    // Decrypt the symmetric key using the private key
    let decrypted_aes_key = decrypt_aes_key_with_rsa(&encrypted_aes_key, &private_key).unwrap();
    let decrypted_aes_iv = decrypt_aes_key_with_rsa(&encrypted_aes_iv, &private_key).unwrap();

    // Decrypt the file using the symmetric key
    decrypt_file(encrypted_file, decrypted_file, &decrypted_aes_key, &decrypted_aes_iv)?;
    println!("File decrypted successfully: {}", decrypted_file);

    Ok(())
}

fn encrypt_file(input_path: &str, output_path: &str, key: &[u8], iv: &[u8]) -> io::Result<()> {
    // Read the input file
    let data = fs::read(input_path)?;

    // Encrypt the data using AES-256-CBC
    let cipher = Cipher::aes_256_cbc();
    let encrypted_data = encrypt(cipher, key, Some(iv), &data).unwrap();

    // Write the encrypted data to the output file
    fs::write(output_path, encrypted_data)?;

    Ok(())
}

fn decrypt_file(input_path: &str, output_path: &str, key: &[u8], iv: &[u8]) -> io::Result<()> {
    // Read the encrypted file
    let encrypted_data = fs::read(input_path)?;

    // Decrypt the data using AES-256-CBC
    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, key, Some(iv), &encrypted_data).unwrap();

    // Write the decrypted data to the output file
    fs::write(output_path, decrypted_data)?;

    Ok(())
}

fn generate_aes_key_and_iv() -> (Vec<u8>, Vec<u8>) {
    let mut rng = rand::thread_rng();
    let aes_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let aes_iv: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    (aes_key, aes_iv)
}

fn encrypt_aes_key_with_rsa(data: &[u8], public_key: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let rsa = Rsa::public_key_from_pem(public_key)?;
    let mut encrypted_data = vec![0; rsa.size() as usize];
    let encrypted_len = rsa.public_encrypt(data, &mut encrypted_data, Padding::PKCS1)?;
    encrypted_data.truncate(encrypted_len);

    Ok(encrypted_data)
}

fn decrypt_aes_key_with_rsa(data: &[u8], private_key: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    let rsa = Rsa::private_key_from_pem(private_key)?;
    let mut decrypted_data = vec![0; rsa.size() as usize];
    let decrypted_len = rsa.private_decrypt(data, &mut decrypted_data, Padding::PKCS1)?;
    decrypted_data.truncate(decrypted_len);

    Ok(decrypted_data)
}