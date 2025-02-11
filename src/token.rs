use odra::{casper_types::U256, prelude::*, Address, SubModule};
use odra_modules::{cep18::utils::Cep18Modality, cep18_token::Cep18};

/// A module definition. Each module struct consists of Vars and Mappings
/// or/and other modules.
#[odra::module]
pub struct Cep18Meme {
    /// A submodule that implements the CEP-18 token standard.
    token: SubModule<Cep18>,
}

/// Module implementation.
///
/// To generate entrypoints,
/// an implementation block must be marked as #[odra::module].
#[odra::module]
impl Cep18Meme {
    /// Initializes the contract with the given metadata and initial supply.
    pub fn init(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let caller = self.env().caller();
        let minter_list = vec![caller];
        self.token.init(
            symbol,
            name,
            decimals,
            initial_supply,
            vec![],
            minter_list,
            Some(Cep18Modality::MintAndBurn),
        );
    }

    // Delegate all Cep18 functions to the token submodule.
    delegate! {
        to self.token {
            /// Admin EntryPoint to manipulate the security access granted to users.
            /// One user can only possess one access group badge.
            /// Change strength: None > Admin > Minter
            /// Change strength meaning by example: If a user is added to both Minter and Admin, they will be an
            /// Admin, also if a user is added to Admin and None then they will be removed from having rights.
            /// Beware: do not remove the last Admin because that will lock out all admin functionality.
            fn change_security(
                &mut self,
                admin_list: Vec<Address>,
                minter_list: Vec<Address>,
                none_list: Vec<Address>
            );

            /// Returns the name of the token.
            fn name(&self) -> String;

            /// Returns the symbol of the token.
            fn symbol(&self) -> String;

            /// Returns the number of decimals the token uses.
            fn decimals(&self) -> u8;

            /// Returns the total supply of the token.
            fn total_supply(&self) -> U256;

            /// Returns the balance of the given address.
            fn balance_of(&self, address: &Address) -> U256;

            /// Returns the amount of tokens the owner has allowed the spender to spend.
            fn allowance(&self, owner: &Address, spender: &Address) -> U256;

            /// Approves the spender to spend the given amount of tokens on behalf of the caller.
            fn approve(&mut self, spender: &Address, amount: &U256);

            /// Decreases the allowance of the spender by the given amount.
            fn decrease_allowance(&mut self, spender: &Address, decr_by: &U256);

            /// Increases the allowance of the spender by the given amount.
            fn increase_allowance(&mut self, spender: &Address, inc_by: &U256);

            /// Transfers tokens from the caller to the recipient.
            fn transfer(&mut self, recipient: &Address, amount: &U256);

            /// Transfers tokens from the owner to the recipient using the spender's allowance.
            fn transfer_from(&mut self, owner: &Address, recipient: &Address, amount: &U256);

            /// Mints new tokens and assigns them to the given address.
            fn mint(&mut self, owner: &Address, amount: &U256);

            /// Burns the given amount of tokens from the given address.
            fn burn(&mut self, owner: &Address, amount: &U256);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odra::host::Deployer;

    #[test]
    fn it_works() {
        let env = odra_test::env();
        let init_args = Cep18MemeInitArgs {
            name: "Cep18Meme".to_string(),
            symbol: "MT".to_string(),
            decimals: 10,
            initial_supply: U256::from(1_000_000_000_000u64),
        };
        assert!(Cep18MemeHostRef::try_deploy(&env, init_args).is_ok());
    }
}
