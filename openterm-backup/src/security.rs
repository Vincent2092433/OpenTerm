use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn help() {
    println!("===============================");
    println!(" SpellShark Security Toolkit");
    println!("===============================");
    println!("security help");
    println!("security hash <text>");
    println!("security encode <text>");
    println!("security decode <base64>");
    println!("security random");
}

pub fn hash(text: &str) {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);

    println!("Hash:");
    println!("{:016x}", hasher.finish());
}

const TABLE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(text: &str) {
    let bytes = text.as_bytes();
    let mut output = String::new();

    let mut i = 0;

    while i < bytes.len() {

        let b0 = bytes[i];

        let b1 = if i + 1 < bytes.len() {
            bytes[i + 1]
        } else {
            0
        };

        let b2 = if i + 2 < bytes.len() {
            bytes[i + 2]
        } else {
            0
        };

        output.push(TABLE[(b0 >> 2) as usize] as char);

        output.push(
            TABLE[(((b0 & 0b00000011) << 4) | (b1 >> 4)) as usize] as char,
        );

        if i + 1 < bytes.len() {
            output.push(
                TABLE[(((b1 & 0b00001111) << 2) | (b2 >> 6)) as usize] as char,
            );
        } else {
            output.push('=');
        }

        if i + 2 < bytes.len() {
            output.push(TABLE[(b2 & 0b00111111) as usize] as char);
        } else {
            output.push('=');
        }

        i += 3;
    }

    println!("{}", output);
}

pub fn decode(_text: &str) {
    println!("Base64 decoding will be added in the next version.");
}

pub fn random() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    println!("{:x}", now.as_nanos());
}