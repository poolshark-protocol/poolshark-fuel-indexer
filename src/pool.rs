extern crate alloc;
use fuel_indexer_macros::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

graphql_schema!("concentrated_liquidity", "../schema/schema.graphql");
wasm_abigen!(
    no_name,
    "../contracts/concentrated_liquidity.json"
);

#[handler]
fn handle_init(event: InitEvent) {
    Logger::info("Pool initialized.");
    let InitEvent {
        contract_id,
        token0,
        token1,
        swap_fee,
        tick_spacing,
        init_price_lower,
        init_price_upper,
        init_tick_underlying
    } = event;

    let mut pool = match Pool::load(contract_id) {
        Some(t) => t,
        None => Pool {
            id: 0,
            token0: ContractId::from(token0),
            token1: ContractId::from(token1),
            swap_fee: swap_fee,
            liquidity: U128 {
                upper: 0,
                lower: 0
            },
            sqrt_price: Q64x64 { 
                integer: init_price_upper,
                decimal: init_price_lower
            },
            fee_growth_global0: Q64x64 {
                integer: 0,
                decimal: 0
            },
            fee_growth_global1: Q64x64 {
                integer: 0,
                decimal: 0
            },
            nearest_tick: I24 {
                underlying: init_tick_underlying
            }
        },
    };

    pool.save();
}