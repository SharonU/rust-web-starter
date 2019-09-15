extern crate serde_json;

#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum AccountEvent {
    Deposit {
        account: u64,
        amount: u32
    },
    Withdraw {
        account: u64,
        amount: u32
    },
    Transfer {
        beneficiary_account: u64,
        originator_account: u64,
        amount: u32
    }
}

#[allow(dead_code)]
impl AccountEvent {
    pub fn deposit(_account: u64, _amount: u32) -> Result<AccountEvent, String> {
        if _account == 0 {
            Err("Account must be non-zero".to_string())
        } else if _amount == 0 {
            Err("Amount must be non-zero".to_string())
        } else {
            Ok(AccountEvent::Deposit {
                account: _account,
                amount: _amount
            })
        }
    }

    pub fn withdraw(_account: u64, _amount: u32) -> Result<AccountEvent, String> {
        if _account == 0 {
            Err("Account must be non-zero".to_string())
        } else if _amount == 0 {
            Err("Amount must be non-zero".to_string())
        } else {
            Ok(AccountEvent::Withdraw {
                account: _account,
                amount: _amount
            })
        }
    }

    pub fn transfer(_beneficiary_account: u64, _originator_account: u64, _amount: u32) -> Result<AccountEvent, String> {
        if _beneficiary_account == 0 {
            Err("Beneficiary account must be non-zero".to_string())
        } else if _originator_account == 0 {
            Err("Originator account must be non-zero".to_string())
        } else if _amount == 0 {
            Err("Amount must be non-zero".to_string())
        } else {
            Ok(AccountEvent::Transfer {
                beneficiary_account: _beneficiary_account,
                originator_account: _originator_account,
                amount: _amount
            })
        }
    }

    pub fn deser(_str: &str) -> Result<AccountEvent, String> {
        match serde_json::from_str(_str) {
            Ok(evt) => Ok(evt),
            Err(_) => Err("Error deserializing event DTO".to_string())
        }
    }

    pub fn ser(_evt : AccountEvent) -> Result<String, String> {
        match serde_json::to_string(&_evt) {
            Ok(str) => Ok(str),
            Err(_) => Err("Error serializing event DTO".to_string())
        }
    }
}
