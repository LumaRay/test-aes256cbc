// https://crates.io/crates/openssl
// Encrypt 2023
// Decrypt 513

use openssl::symm::*;

use rand::{ Rng, OsRng };

use std::time::{SystemTime};//, UNIX_EPOCH};

const LOOPS_COUNT: u32 = 1_000_000;

fn main() {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let mut message: [u8; 1024] = [0; 1024]; //"Hello World!";
    

    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    rng.fill_bytes(&mut message);




    let block_size = Cipher::aes_256_cbc().block_size();

    let mut ciphertext = vec![0; message.len() + block_size];

    // Encrypt 2 chunks of plaintexts successively.
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        //ciphertext = vec![0; message.len() + block_size];
        // Create a cipher context for encryption.
        let mut encrypter = Crypter::new(
            Cipher::aes_256_cbc(),
            Mode::Encrypt,
            &key,
            Some(&iv)).unwrap();
        let mut count = encrypter.update(&message, &mut ciphertext).unwrap();
        //let mut count = encrypter.update(plaintexts[0], &mut ciphertext).unwrap();
        //count += encrypter.update(plaintexts[1], &mut ciphertext[count..]).unwrap();
        count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
        //ciphertext.truncate(count);
    }
    println!("Encrypt {:?}", SystemTime::now().duration_since(start).unwrap());


    // Let's pretend we don't know the plaintext, and now decrypt the ciphertext.
    let data_len = ciphertext.len();
    //let ciphertexts = [&ciphertext[..9], &ciphertext[9..]];

    let mut plaintext = vec![0; data_len + block_size];

    // Decrypt 2 chunks of ciphertexts successively.
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        //plaintext = vec![0; data_len + block_size];
        // Create a cipher context for decryption.
        let mut decrypter = Crypter::new(
            Cipher::aes_256_cbc(),
            Mode::Decrypt,
            &key,
            Some(&iv)).unwrap();
        let mut count = decrypter.update(&ciphertext, &mut plaintext).unwrap();
        //let mut count = decrypter.update(ciphertexts[0], &mut plaintext).unwrap();
        //count += decrypter.update(ciphertexts[1], &mut plaintext[count..]).unwrap();
        count += decrypter.finalize(&mut plaintext[count..]).unwrap();
        //plaintext.truncate(count);
        //assert_eq!(plaintext, &message);
    }
    println!("Decrypt {:?}", SystemTime::now().duration_since(start).unwrap());



    /*let cipher = Cipher::aes_256_cbc();
    let mut e = Crypter::new(cipher, Mode::Encrypt, &key, Some(&iv)).unwrap();
    let mut enc_data = vec![0; message.len() + cipher.block_size()];
    e.update(&message, &mut enc_data).unwrap();
    let len = e.finalize(&mut enc_data).unwrap();
    println!("{:?}", len);


    let mut d = Crypter::new(cipher, Mode::Decrypt, &key, Some(&iv)).unwrap();
    let mut dec_data = vec![0; enc_data.len() + cipher.block_size()];
    d.update(&enc_data, &mut dec_data).unwrap();
    let len = d.finalize(&mut dec_data).unwrap();
    //dec_data.truncate(len);
    println!("{:?}", dec_data);
    //println!("{:?}", String::from_utf8_lossy(&dec_data));*/

    /*let key = openssl::pkcs5::bytes_to_key(cipher, openssl::hash::MessageDigest::md5(), KEY, Some(&message_d[8..16]), 1).unwrap();
    let new_data = decrypt(cipher, &key.key, key.iv.as_deref(), & message_d[16..]).unwrap();
    println!("{}", new_data.iter().map(|e| *e as char).collect::<String>());*/
}
