// https://crates.io/crates/cbc
// Encrypt 2046
// Decrypt 960

use std::time::{SystemTime};//, UNIX_EPOCH};

use aes;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
//use hex_literal::hex;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

use rand::{ Rng, OsRng };

const LOOPS_COUNT: u32 = 1_000_000;

fn main() {
    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];
    let mut message: [u8; 1024] = [0; 1024]; //"Hello World!";
    

    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    rng.fill_bytes(&mut message);


// Encrypt
    /*let mut encrypted_data = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut message, 1024)
        .unwrap();*/
    let mut buf: [u8; 1024*2] = [0; 1024*2];
    let mut ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&message, &mut buf)
        .unwrap();
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&message, &mut buf)
        .unwrap();
    }
    println!("Encrypt {:?}", SystemTime::now().duration_since(start).unwrap());

// Decrypt
    //println!("{:?}", encrypted_data);
    /*let decrypted_data = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut *encrypted_data.clone())
        .unwrap();*/
    let mut buf: [u8; 1024*2] = [0; 1024*2];
    let mut pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
        .unwrap();
    assert_eq!(pt, &message);
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
        .unwrap();
    }
    println!("Decrypt {:?}", SystemTime::now().duration_since(start).unwrap());

    //let key = [0x42; 16];
    //let iv = [0x24; 16];
    //let plaintext = *b"hello world! this is my plaintext.";

    /*let ciphertext = hex!(
        "c7fe247ef97b21f07cbdd26cb5d346bf"
        "d27867cb00d9486723e159978fb9a5f9"
        "14cfb228a710de4171e396e7b6cf859e"
    );*/

    // encrypt/decrypt in-place
    // buffer must be big enough for padded plaintext
    /*let ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);


    let pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext);*/

    // encrypt/decrypt from buffer to buffer
    /*let mut buf = [0u8; 48];
    let ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut buf)
        .unwrap();
    assert_eq!(ct, &ciphertext[..]);

    let mut buf = [0u8; 48];
    let pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_b2b_mut::<Pkcs7>(&ct, &mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext);*/
}


