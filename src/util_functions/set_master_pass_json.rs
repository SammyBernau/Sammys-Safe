use crate::chacha20poly1305_encrypt_string;
use crate::util_functions::check_for_account::get_account_list;

pub fn set_master_pass_json(master_pass: String) {

    let (encrypted_master_pass, encrypted_master_pass_tag) = chacha20poly1305_encrypt_string(
        master_pass.clone(),
        master_pass.clone());


    let mut account_list = get_account_list();
    account_list.account_list[0].password = encrypted_master_pass.clone();
    account_list.account_list[0].tag = encrypted_master_pass_tag.clone();

    std::fs::write("password_manager_json.json", serde_json::to_string(&account_list).unwrap()).expect("File save failed");

}