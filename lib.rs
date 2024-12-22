// Import the necessary components and macros from the elrond_wasm crate.
// This is essential for developing smart contracts on the MultiversX blockchain.
elrond_wasm::imports!();

// Declare a separate module named `contract`.
// This module will contain the core logic and implementation details of the smart contract.
mod contract;

// Define the main trait for the smart contract using the elrond_wasm::contract attribute macro.
// The trait specifies the contract's interface, including its endpoints and view functions.
#[elrond_wasm::contract]
pub trait WinterTokenStaking {
    // The `init` function is a special initializer function.
    // It is called once when the contract is deployed to the blockchain.
    // This function is typically used to set up initial state or configurations.
    #[init]
    fn init(&self) {}

    // Define an endpoint named `stake_token_winter`.
    // Endpoints are functions that users or other contracts can call to interact with this contract.
    #[endpoint(stakeTokenWinter)]
    fn stake_token_winter(
        &self, 
        // The `#[payment_token]` attribute indicates that this parameter should match the token being sent with the transaction.
        #[payment_token] token: TokenIdentifier, 
        // The `#[payment_amount]` attribute ensures that this parameter corresponds to the amount of tokens being transferred.
        #[payment_amount] amount: BigUint
    ) {
        // Call the `stake_token_winter` function in the `contract` module.
        // This delegates the implementation logic to the `contract` module, keeping the trait clean and focused on the interface.
        contract::stake_token_winter(self, token, amount);
    }

    // Define a view function named `get_staked_tokens`.
    // View functions are read-only and do not modify the contract's state. They are used to retrieve data.
    #[view(getStakedTokens)]
    fn get_staked_tokens(
        &self, 
        // The address parameter specifies the account whose staked tokens are being queried.
        address: ManagedAddress
    ) -> contract::StakedTokens<Self::Api> {
        // Call the `get_staked_tokens` function in the `contract` module.
        // This delegates the logic for retrieving staked tokens to the `contract` module.
        contract::get_staked_tokens(self, address)
    }
}