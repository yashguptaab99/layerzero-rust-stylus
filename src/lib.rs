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
    error PropagationError(string result);
}

sol_interface! {
    interface ICrossChainMessenger {
        function send(uint32 _dst_eid, string calldata _message) external payable;
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
        /// Mapping from user addresses to their respective balances.
        /// 
        /// This mapping keeps track of the token balances for each user address.
        mapping(address => uint256) balances;

        /// Address of the cross-chain messenger.
        /// 
        /// This address is used to interact with the cross-chain messaging system.
        address cross_chain_messenger;
    }
}

#[public]
impl Balances {

    /// Initializes the contract with the given messenger address.
    ///
    /// # Arguments
    ///
    /// * `messenger_address` - The address of the messenger to be set.
    ///
    /// # Returns
    ///
    /// * `Result<(), ContractErrors>` - Returns `Ok(())` if the initialization is successful,
    ///   otherwise returns an error of type `ContractErrors`.
    ///
    /// # Errors
    ///
    /// * `ContractErrors::InvalidEndpoint` - If the provided `messenger_address` is zero.
    /// ```
    pub fn init(&mut self, messenger_address: Address) -> Result<(), ContractErrors> {
        if messenger_address.is_zero() {
            return Err(ContractErrors::InvalidEndpoint(InvalidEndpoint {}));
        }
        self.cross_chain_messenger.set(messenger_address);
        Ok(())
    }

    /// Allows a user to deposit a specified amount of tokens into the contract.
    ///
    /// # Arguments
    ///
    /// * `quote` - The amount of tokens to be quoted for the deposit.
    ///
    /// # Returns
    ///
    /// * `Result<(), ContractErrors>` - Returns `Ok(())` if the deposit is successful,
    ///   otherwise returns an error of type `ContractErrors`.
    ///
    /// # Errors
    ///
    /// * `ContractErrors::InvalidDepositAmount` - If the deposited amount is zero.
    /// * `ContractErrors::InvalidEndpoint` - If the cross-chain messenger address is zero.
    /// * `ContractErrors::PropagationError` - If there is an error propagating the message.
    /// ```
    #[payable]
    pub fn deposit(&mut self, quote: U256) -> Result<(), ContractErrors> {
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

        let messenger: ICrossChainMessenger =
            ICrossChainMessenger::new(*self.cross_chain_messenger);
        let message: String = format!(
            "User: {:?}, Deposited Amount: {:?}",
            msg::sender(),
            msg::value() - quote
        );
        let result: Result<(), Error> = messenger.send(Call::new_in(self).value(quote), 40267, message);

        match result {
            Ok(_) => Ok(()),

            Err(_) => {
                return Err(ContractErrors::PropagationError(PropagationError { result: format!("{:?}", result.err().unwrap()) }));   
            }
        }
    }

    /// Retrieves the balance of a specified user.
    ///
    /// # Arguments
    ///
    /// * `user` - The address of the user whose balance is to be retrieved.
    ///
    /// # Returns
    ///
    /// * `U256` - The balance of the specified user.
    /// ```
    pub fn get_balance(&self, user: Address) -> U256 {
        self.balances.get(user)
    }

    /// Retrieves the address of the cross-chain messenger.
    ///
    /// # Returns
    ///
    /// * `Address` - The address of the cross-chain messenger.
    /// ```
    pub fn get_cross_chain_messenger(&self) -> Address {
        self.cross_chain_messenger.get()
    }
}
