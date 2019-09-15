use nats_connection::pooled_connection::NatsConnection;
use rocket_contrib::json::Json;
use test_shared::*;

#[derive(Deserialize)]
pub struct Deposit {
    pub account: u64,
    pub amount: u32
}

#[derive(Deserialize)]
pub struct Withdraw {
    pub account: u64,
    pub amount: u32
}

#[derive(Deserialize)]
pub struct Transfer {
    pub beneficiary_account: u64,
    pub originator_account: u64,
    pub amount: u32
}

#[catch(503)]
pub fn service_not_available() -> &'static str {
    "Service is not available."
}

#[post("/deposit", format = "json", data = "<_deposit>")]
pub fn deposit(_deposit: Json<Deposit>, mut _conn: NatsConnection) -> String {
    let deposit : Deposit = _deposit.into_inner();

    match AccountEvent::deposit(deposit.account, deposit.amount) {
        Err(msg) => msg,
        Ok(event) => {
            let serialized = AccountEvent::ser(event).unwrap();
            _conn.publish("events", serialized.as_bytes()).unwrap();
            format!("Deposit command sent")
        }
    }
}

#[post("/withdraw", format = "json", data = "<_withdraw>")]
pub fn withdraw(_withdraw: Json<Withdraw>, mut _conn: NatsConnection) -> String {
    let withdraw : Withdraw = _withdraw.into_inner();

    match AccountEvent::withdraw(withdraw.account, withdraw.amount) {
        Err(msg) => msg,
        Ok(event) => {
            let serialized = AccountEvent::ser(event).unwrap();
            _conn.publish("events", serialized.as_bytes()).unwrap();
            format!("Withdraw command sent")
        }
    }
}

#[post("/transfer", format = "json", data = "<_transfer>")]
pub fn transfer(_transfer: Json<Transfer>, mut _conn: NatsConnection) -> String {
    let transfer : Transfer = _transfer.into_inner();

    match AccountEvent::transfer(transfer.beneficiary_account, transfer.originator_account, transfer.amount) {
        Err(msg) => msg,
        Ok(event) => {
            let serialized = AccountEvent::ser(event).unwrap();
            _conn.publish("events", serialized.as_bytes()).unwrap();
            format!("Transfer command sent")
        }
    }
}
