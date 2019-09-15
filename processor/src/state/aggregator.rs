extern crate test_shared;
use test_shared::*;
use std::collections::HashMap;

pub trait State {
    type Item;

    fn generation(&self) -> u64;
    fn apply(self, evt: &Self::Item) -> Self where Self:Sized;
}

#[derive(Debug)]
pub struct AggregateState {
    generation: u64,
    balances: HashMap<u64, u32>
}

impl AggregateState {
    pub fn new() -> AggregateState {
        AggregateState {
            generation: 1,
            balances: HashMap::new()
        }
    }

    pub fn get_balances(&self) -> &HashMap<u64, u32> {
        &self.balances
    }
}

impl State for AggregateState {
    type Item = AccountEvent;

    fn generation(&self) -> u64 {
        self.generation
    }

    fn apply(mut self, evt: &AccountEvent) -> AggregateState {
        self.generation += 1;

        match evt {
            &AccountEvent::Deposit { amount: _amount, account: _account } => {
                let new_balance = self.balances
                    .get(&_account)
                    .unwrap_or(&0u32)
                    .checked_add(_amount)
                    .unwrap_or(u32::max_value());

                self.balances.insert(
                    _account,
                    new_balance,
                );
            },
            &AccountEvent::Withdraw { amount: _amount, account: _account } => {
                let new_balance = match self.balances.get(&_account) {
                    Some (&current) if current < _amount => current,
                    Some (&current) => current - _amount,
                    None => 0u32
                };

                self.balances.insert(
                    _account,
                    new_balance,
                );
            },
            &AccountEvent::Transfer { amount: _amount, beneficiary_account: _beneficiary_account, originator_account: _originator_account } => {
                match self.balances.get(&_originator_account) {
                    Some (&originator_current) if originator_current < _amount => {},
                    Some (&originator_current) => {
                        self.balances.insert(
                            _originator_account,
                            originator_current - _amount,
                        );

                        let beneficiary_current = match self.balances.get(&_beneficiary_account) {
                            Some (&beneficiary_current) => beneficiary_current,
                            None => 0u32
                        };

                        self.balances.insert(
                            _beneficiary_account,
                            beneficiary_current + _amount,
                        );
                    },
                    None => {}
                };
            }
        };

        self
    }
}


// TODO:
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_withdraw_with_zero_balance() {
//     }
// }
