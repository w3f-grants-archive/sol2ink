#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0-beta
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod struct_contract {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::{
        AccountId,
        Storage,
        String,
    };
    use scale::{
        Decode,
        Encode,
    };
    use struct_contract::*;


    #[ink(event)]
    pub struct Log {
        #[ink(topic)]
        sender: AccountId,
        message: String,
        priority: u8,
        status: Status,
    }

    #[ink(event)]
    pub struct AnotherLog {}

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct StructContractContract {
        #[storage_field]
        data: impls::Data,
    }

    impl StructContract for StructContractContract {}

    impl struct_contract::Internal for StructContractContract {
        fn _emit_log(&self, sender: AccountId, message: String, priority: u8, status: Status) {
            self.env().emit_event(Log {
                sender,
                message,
                priority,
                status,
            });
        }

        fn _emit_another_log(&self) {
            self.env().emit_event(AnotherLog {});
        }

    }

    impl StructContractContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {})
        }

    }
}