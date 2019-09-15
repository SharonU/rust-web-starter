#![feature(plugin, proc_macro_hygiene, decl_macro, never_type)]

extern crate nats;
extern crate test_shared;

mod state;

use test_shared::*;
use std::env;
use nats::*;
use state::aggregator::*;

fn nats_url() -> String {
    env::var("NATS_ADDRESS")
        .expect("NATS_ADDRESS environment variable must be set")
}

fn main() {
    let mut state = AggregateState::new();

    let mut client = Client::new(nats_url()).unwrap();
    client.subscribe("events", None).unwrap();

    loop {
        let raw_event = client.wait().unwrap();
        let msg : String = String::from_utf8(raw_event.msg).unwrap();

        let domain_event : AccountEvent = AccountEvent::deser(&msg).unwrap();

        state = state.apply(&domain_event);

        println!("\n\ncurrent balances: {:?}\n\n", state.get_balances());
    }
}
