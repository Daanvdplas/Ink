#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod adder {
    use ink::env::{
        call::{build_call, Call, ExecutionInput, Selector},
        CallFlags, DefaultEnvironment,
    };
    /// Increments the underlying `accumulator` value.
    #[ink(storage)]
    pub struct Adder {
        /// The `accumulator` to store the value.
        acc_contract: AccountId,
    }

    impl Adder {
        /// Creates a new `adder` from the given `accumulator`.
        #[ink(constructor)]
        pub fn new(acc_contract: AccountId) -> Self {
            Self { acc_contract }
        }

        /// Increases the `accumulator` value by some amount.
        #[ink(message, selector = 0xC0DECAFE)]
        pub fn inc(&mut self, by: i32) {
            let method_selector = [0xC0, 0xDE, 0xCA, 0xFE];
            let _result = build_call::<<Self as ::ink::env::ContractEnv>::Env>()
                .call(self.acc_contract)
                .call_flags(CallFlags::default())
                .exec_input(ExecutionInput::new(method_selector.into()).push_arg(by))
                .returns::<()>()
                .try_invoke();
        }
    }
}
