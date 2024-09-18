use cep18_meme::token::{Cep18MemeHostRef, Cep18MemeInitArgs};
use odra::{
    casper_types::U256,
    host::{Deployer, HostRef, HostRefLoader},
    Address,
};

pub fn main() {
    println!("[x] Deploying Cep18 contract...");
    let env = odra_casper_livenet_env::env();
    let deployer = env.get_account(0);
    let one_token = U256::from(10).pow(U256::from(13));

    env.set_caller(deployer);
    env.set_gas(230_000_000_000);
    let mut contract = Cep18MemeHostRef::deploy(
        &env,
        Cep18MemeInitArgs {
            symbol: String::from("MEMET"),
            name: String::from("MEMETest"),
            decimals: 13,
            initial_supply: U256::from(42_000_000_000u64)
                .checked_mul(one_token)
                .unwrap(),
        },
    );

    // let address = Address::new("hash-064acd1f559c3fe3f25b8a3eea771d1771477f1e51d48c56e3131fbe8b5ba6d7").unwrap();
    // let mut contract = Cep18MemeHostRef::load(&env, address);

    println!("[x] Contract deployed at: {:?}", contract.address());
    println!("[x] Deployer balance: {:?}", contract.balance_of(&deployer));

    println!("[x] Minting 1_000_000_000 MEMET...");

    env.set_gas(20_000_000_000);
    contract.mint(&deployer, &U256::from(1_000_000_000u64));

    println!("[x] Deployer balance: {:?}", contract.balance_of(&deployer));
}
