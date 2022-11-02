use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use libsql_wasm_abi::*;

#[libsql_bindgen::libsql_bindgen]
pub fn encrypt(data: String, key: String) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_str_to_base64(data)
}

#[libsql_bindgen::libsql_bindgen]
pub fn decrypt(data: String, key: String) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.decrypt_base64_to_string(data).unwrap()
}
