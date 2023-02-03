#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

// Generated with Sol2Ink v2.0.0
// https://github.com/727-Ventures/sol2ink

#[openbrush::contract]
pub mod uniswap_v_2_router_01 {
    use generated::*;
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::SpreadAllocate;
    use openbrush::traits::Storage;


    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct UniswapV2Router01Contract {
        #[storage_field]
        data: impls::Data,
    }

    impl UniswapV2Router01 for UniswapV2Router01Contract {}

    impl generated::impls::uniswap_v_2_router_01::Internal for UniswapV2Router01Contract {}

    impl IUniswapV2Router01 for UniswapV2Router01Contract {}

    impl UniswapV2Router01Contract {
        #[ink(constructor)]
        pub fn new(factory: AccountId, weth: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.data.factory = factory;
                instance.data.weth = weth;
            })
        }

    }
}