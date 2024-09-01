extern crate rand;
extern crate rsa;

use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

/// RSA key pair generation (private and public keys)
fn generate_keys(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to create private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// Encrypting a message using the public key
fn encrypt_message(public_key: &RsaPublicKey, message: &str) -> Vec<u8> {
    let mut rng = OsRng;
    let message_bytes = message.as_bytes();
    public_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            &message_bytes,
        )
        .expect("Failed to encrypt message")
}

/// Decrypting a message using the private key
fn decrypt_message(private_key: &RsaPrivateKey, encrypted_message: &[u8]) -> String {
    let decrypted_message = private_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), encrypted_message)
        .expect("Failed to decrypt message");

    String::from_utf8(decrypted_message).expect("Message is not a valid UTF-8 string")
}

fn main() {
    // RSA key generation
    let (private_key, public_key) = generate_keys(2048);

    // The message we want to encrypt
    let message = "Secret message";

    // Encrypting the message
    let encrypted_message = encrypt_message(&public_key, message);
    println!("Encrypted message: {:?}", encrypted_message);

    // Decrypting the message
    let decrypted_message = decrypt_message(&private_key, &encrypted_message);
    println!("Decrypted message: {}", decrypted_message);
}
