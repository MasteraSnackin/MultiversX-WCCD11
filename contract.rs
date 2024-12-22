// Import necessary types and traits from the elrond_wasm crate.
// BigUint is used for handling large numbers, TokenIdentifier for identifying tokens,
// SingleValueMapper for managing contract storage, and ManagedTypeApi for API types.
use elrond_wasm::types::{BigUint, TokenIdentifier};
use elrond_wasm::storage::mappers::SingleValueMapper;
use elrond_wasm::api::ManagedTypeApi;

// Define a structure to represent the staked tokens of a user.
// This structure holds the amount of tokens and the epoch until which they are locked.
pub struct StakedTokens<M: ManagedTypeApi> {
    pub amount: BigUint<M>,         // The amount of tokens that have been staked.
    pub lock_until_epoch: u64,      // The epoch number until which the tokens are locked.
}

// Implement the logic for staking tokens.
// This function is called when users interact with the contract to stake their tokens.
pub fn stake_token_winter<M: ManagedTypeApi>(
    contract: &impl WinterTokenStaking<M>, // Reference to the contract implementation.
    token: TokenIdentifier,                // The identifier of the token being staked.
    amount: BigUint<M>,                    // The amount of tokens to stake.
) {
    // Get the address of the caller (the person or entity invoking the contract).
    let caller = contract.blockchain().get_caller();
    // Get the current epoch from the blockchain.
    let current_epoch = contract.blockchain().get_block_epoch();

    // Ensure that the correct token is being staked and the amount is valid.
    require!(token == contract.winter_token_id().get(), "Invalid token");
    require!(amount > 0, "Staking amount must be greater than zero");

    // Calculate the epoch until which the tokens will be locked.
    let lock_until_epoch = current_epoch + contract.min_staking_epochs().get();
    // Update the staked tokens for the caller's address.
    contract.staked_tokens(&caller).update(|staked| {
        staked.amount += amount;                // Increment the staked amount.
        staked.lock_until_epoch = lock_until_epoch; // Set the lock expiration epoch.
    });
}

// Function to retrieve the staked tokens for a specific address.
// This is a view function that allows users to query their staking status.
pub fn get_staked_tokens<M: ManagedTypeApi>(
    contract: &impl WinterTokenStaking<M>, // Reference to the contract implementation.
    address: ManagedAddress,               // The address for which to retrieve staked tokens.
) -> StakedTokens<M> {
    // Access the storage and return the staked tokens associated with the given address.
    contract.staked_tokens(&address).get()
}

// Define a trait that outlines the necessary methods and associated types for the staking contract.
pub trait WinterTokenStaking<M: ManagedTypeApi> {
    // Method to access blockchain-related functionalities, such as getting the caller and current epoch.
    fn blockchain(&self) -> &dyn elrond_wasm::api::BlockchainApi<M>;

    // Method to access the storage mapper for a user's staked tokens.
    fn staked_tokens(&self, address: &ManagedAddress) ->ValueMapper<StakedTokens<M>>    // Method to access the storage mapper for the token identifier used in staking.
    fn winter_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // Method to access the storage mapper for the minimum number of epochs tokens should be staked.
    fn min_staking_epochs(&self) -> SingleValueMapper<u64>;
}