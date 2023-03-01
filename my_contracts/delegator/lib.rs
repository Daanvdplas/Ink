#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod caller {
    use ink::env::{
        call::{build_call, Call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    };

    /// Specifies the state of the `delegator` contract.
    ///
    /// In `Adder` state the `delegator` contract will delegate to the `Adder` contract
    /// and in `Subber` state will delegate to the `Subber` contract.
    ///
    /// The initial state is `Adder`.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub enum Which {
        Adder,
        Subber,
    }

    /// Delegates calls to an `adder` or `subber` contract to mutate
    /// a value in an `accumulator` contract.
    ///
    /// # Note
    ///
    /// In order to instantiate the `delegator` smart contract we first
    /// have to manually put the code of the `accumulator`, `adder`
    /// and `subber` smart contracts, receive their code hashes from
    /// the signalled events and put their code hash into our
    /// `delegator` smart contract.
    ///
    /// The `AccumulatorRef`, `AdderRef` and `SubberRef` are smart contract
    /// reference types that have been automatically generated by ink!.
    #[ink(storage)]
    pub struct Delegator {
        /// Says which of `adder` or `subber` is currently in use.
        which: Which,
        /// The `accumulator` smart contract.
        acc_contract: AccountId,
        /// The `adder` smart contract.
        add_contract: AccountId,
        /// The `subber` smart contract.
        sub_contract: AccountId,
    }

    impl Delegator {
        #[ink(constructor)]
        pub fn new(
            acc_contract: AccountId,
            add_contract: AccountId,
            sub_contract: AccountId,
        ) -> Self {
            Delegator {
                which: Which::Adder,
                acc_contract,
                add_contract,
                sub_contract,
            }
        }

        #[ink(message)]
        pub fn get(&self) {
            let method_selector = [0xC0, 0xDE, 0xCA, 0xF1];
            let _result = build_call::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(self.acc_contract)
                .call_flags(CallFlags::default())
                .exec_input(ExecutionInput::new(method_selector.into()))
                .returns::<()>()
                .try_invoke();
        }

        #[ink(message)]
        pub fn change(&self, by: i32) {
            let method_selector = [0xC0, 0xDE, 0xCA, 0xFE];
            let contract = match self.which {
                Which::Adder => self.add_contract,
                Which::Subber => self.sub_contract,
            };
            let _result = build_call::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(contract)
                .call_flags(CallFlags::default())
                .exec_input(ExecutionInput::new(method_selector.into()).push_arg(by))
                .returns::<()>()
                .try_invoke();
        }

        #[ink(message)]
        pub fn switch(&mut self) {
            match self.which {
                Which::Adder => {
                    self.which = Which::Subber;
                }
                Which::Subber => {
                    self.which = Which::Adder;
                }
            }
        }
    }
}

// /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
// ///
// /// When running these you need to make sure that you:
// /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
// /// - Are running a Substrate node which contains `pallet-contracts` in the background
// #[cfg(all(test, feature = "e2e-tests"))]
// mod e2e_tests {
//     /// Imports all the definitions from the outer scope so we can use them here.
//     use super::*;

//     /// A helper function used for calling contract messages.
//     use ink_e2e::build_message;

//     /// The End-to-End test `Result` type.
//     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

//     /// We test that we can upload and instantiate the contract using its default constructor.
//     #[ink_e2e::test]
//     async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
//         // Given
//         let constructor = DelegatorRef::default();

//         // When
//         let contract_account_id = client
//             .instantiate("delegator", &ink_e2e::alice(), constructor, 0, None)
//             .await
//             .expect("instantiate failed")
//             .account_id;

//         // Then
//         let get = build_message::<DelegatorRef>(contract_account_id.clone())
//             .call(|delegator| delegator.get());
//         let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
//         assert!(matches!(get_result.return_value(), false));

//         Ok(())
//     }

//     /// We test that we can read and write a value from the on-chain contract contract.
//     #[ink_e2e::test]
//     async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
//         // Given
//         let constructor = DelegatorRef::new(false);
//         let contract_account_id = client
//             .instantiate("delegator", &ink_e2e::bob(), constructor, 0, None)
//             .await
//             .expect("instantiate failed")
//             .account_id;

//         let get = build_message::<DelegatorRef>(contract_account_id.clone())
//             .call(|delegator| delegator.get());
//         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//         assert!(matches!(get_result.return_value(), false));

//         // When
//         let flip = build_message::<DelegatorRef>(contract_account_id.clone())
//             .call(|delegator| delegator.flip());
//         let _flip_result = client
//             .call(&ink_e2e::bob(), flip, 0, None)
//             .await
//             .expect("flip failed");

//         // Then
//         let get = build_message::<DelegatorRef>(contract_account_id.clone())
//             .call(|delegator| delegator.get());
//         let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
//         assert!(matches!(get_result.return_value(), true));

//         Ok(())
//     }
// }