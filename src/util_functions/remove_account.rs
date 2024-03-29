use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::json_path::get_json_path;
use crate::AccountList;

pub fn remove_account(website: String) -> std::io::Result<()> {
    if check_account_existence(&website) {
        //let json_file = "password_manager_json.json";
        let mut entire_account_list = {
            let json_output = std::fs::read_to_string(get_json_path())?;

            //Loads the AccountList structure from the string
            serde_json::from_str::<AccountList>(&json_output).unwrap()
        };

        let mut index = 0;
        for elem in &entire_account_list.account_list {
            if elem.website == website {
                //Adds the new account to the json formatted array
                entire_account_list.account_list.remove(index);
                std::fs::write(
                    get_json_path(),
                    serde_json::to_string(&entire_account_list).unwrap(),
                )?;
                break;
            }
            index += 1;
        }
    }
    Ok(())
}
