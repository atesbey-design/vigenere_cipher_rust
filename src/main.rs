fn encrypt_vigenere(plain_text: &str, key: &str) -> String {
    let mut cipher_text = String::new();
    let key = key.chars().cycle(); 
    
    for (i, c) in plain_text.chars().enumerate() {
        let key_char = key.clone().nth(i).unwrap();
        let key_offset = (key_char as u8).to_ascii_lowercase() - b'a';
        let cipher_char = ((c as u8).to_ascii_lowercase() + key_offset - b'a') % 26 + b'a';
        cipher_text.push(char::from_u32(cipher_char.into()).unwrap());
    }
    
    cipher_text
}

fn decrypt_vigenere(cipher_text: &str, key: &str) -> String {
    let mut plain_text = String::new();
    let key = key.chars().cycle(); 
    
    for (i, c) in cipher_text.chars().enumerate() {
        let key_char = key.clone().nth(i).unwrap();
        let key_offset = (key_char as u8).to_ascii_lowercase() - b'a';
        let plain_char = ((c as u8).to_ascii_lowercase() - key_offset - b'a' + 26) % 26 + b'a';
        plain_text.push(char::from_u32(plain_char.into()).unwrap());
    }
    
    plain_text
}


fn main() {
    let plaintext = String::from("ATES");
    let key = String::from("ETH");

    let ciphertext = encrypt_vigenere(&plaintext, &key);
    println!("Şifreli metin: {}", ciphertext);

    let decryptedtext = decrypt_vigenere(&ciphertext, &key);
    println!("Çözülmüş metin: {}", decryptedtext);
}