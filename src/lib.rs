#![cfg_attr(not(feature = "export-abi"), no_main)]

extern crate alloc;

extern crate stylus_sdk;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::sol,
    call::{Call, Error},
    msg,
    prelude::*,
};

sol! {
    error InvalidDepositAmount();
    error InvalidEndpoint();
    error PropagationError();
}

sol_interface! {
    interface ICrossChainMessenger {
        function propagate(string _message) external;
    }
}

#[derive(SolidityError)]
pub enum ContractErrors {
    InvalidDepositAmount(InvalidDepositAmount),
    InvalidEndpoint(InvalidEndpoint),
    PropagationError(PropagationError),
}

sol_storage! {
    #[entrypoint]
    pub struct Balances {
        mapping(address => uint256) balances;
        address cross_chain_messenger;
    }
}

#[public]
impl Balances {
    pub fn init(&mut self, messenger_address: Address) -> Result<(), ContractErrors> {
        if messenger_address.is_zero() {
            return Err(ContractErrors::InvalidEndpoint(InvalidEndpoint {}));
        }
        self.cross_chain_messenger.set(messenger_address);
        Ok(())
    }

    #[payable]
    pub fn deposit(&mut self) -> Result<(), ContractErrors> {
        if msg::value().is_zero() {
            return Err(ContractErrors::InvalidDepositAmount(
                InvalidDepositAmount {},
            ));
        }

        if self.cross_chain_messenger.get().is_zero() {
            return Err(ContractErrors::InvalidEndpoint(InvalidEndpoint {}));
        }

        let mut user_balance = self.balances.setter(msg::sender());
        let current_balance: U256 = user_balance.get();
        user_balance.set(current_balance + msg::value());

        let messenger: ICrossChainMessenger = ICrossChainMessenger::new(*self.cross_chain_messenger);
        let message: String = format!(
            "User: {:?}, Deposited Amount: {:?}",
            msg::sender(),
            msg::value()
        );
        let result: Result<(), Error> = messenger.propagate(Call::new_in(self), message);

        match result {
            Ok(_) => Ok(()),

            Err(_) => {
                return Err(ContractErrors::PropagationError(PropagationError {}));
            }
        }
    }

    pub fn get_balance(&self, user: Address) -> U256 {
        self.balances.get(user)
    }
}
