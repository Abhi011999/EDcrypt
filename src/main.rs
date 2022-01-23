#![allow(non_snake_case)]

use std::env;
use std::fs;

use reqwest;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use tokio;

const API_ENCRYPT_URL: &str = "https://classify-web.herokuapp.com/api/encrypt";
const API_DECRYPT_URL: &str = "https://classify-web.herokuapp.com/api/decrypt";
const MESSAGE: &str = "I really want this internship!";

#[derive(Serialize, Debug)]
struct DATA {
    data: String,
    key: String,
}

#[derive(Deserialize, Debug)]
struct RESULT {
    result: String,
}

fn extract_key(filename: String) -> String {
    let key_file = fs::read_to_string(filename).unwrap();
    let key_iter = key_file.split_whitespace();

    let mut key = "".to_string();
    let mut i = 0;

    for l in key_iter {
        if i > 7 && i < 22 {
            key.push_str(l);
        }
        i += 1;
    }

    return key;
}

async fn encrypt(plaintext: String, key: String) -> Result<RESULT, Error> {
    let data = DATA {
        data: plaintext,
        key: key,
    };

    let client = reqwest::Client::new();
    let res = client
        .post(API_ENCRYPT_URL)
        .json(&data)
        .send()
        .await?
        .json::<RESULT>()
        .await?;

    return Ok(res);
}

async fn decrypt(ciphertext: String, key: String) -> Result<RESULT, Error> {
    let data = DATA {
        data: ciphertext,
        key: key,
    };
    
    let client = reqwest::Client::new();
    let res = client
        .post(API_DECRYPT_URL)
        .json(&data)
        .send()
        .await?
        .json::<RESULT>()
        .await?;

    return Ok(res);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let key_filename = &args[1];

    println!("\nExtracting key from keyfile...\n");
    let key = extract_key(key_filename.to_string());

    println!("Encrypting...");
    let encrypted_message = encrypt(MESSAGE.to_string(), key.to_string()).await?.result;
    println!("Encrypted message - {}\n", encrypted_message);
    
    println!("Decrypting...");
    let decrypted_message = decrypt(encrypted_message, key.to_string()).await?.result;
    println!("Decrypted message - {}\n", decrypted_message);

    Ok(())
}
