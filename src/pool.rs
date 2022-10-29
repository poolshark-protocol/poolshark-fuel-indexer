extern crate alloc;
use fuel_indexer_macros::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

graphql_schema!("concentrated_liquidity", "schema/schema.graphql");
wasm_abigen!(
    no_name,
    "hello-indexer/contracts/hello_indexer.json"
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
        init_tick
    } = event;

    let mut pool = match Pool::load(contract_id) {
        Some(t) => t,
        None => Pool {
            id: 0,
            token0: ContractId::from(token0),
            token1: ContractId::from(token1),
            swap_fee: swap_fee,
            liquidity: U128::from(0,0),
            sqrt_price: Q64x64::from(init_price_upper, init_price_lower),
            fee_growth_global0: Q64x64::from(0,0),
            fee_growth_global1: Q64x64::from(0,0),
            nearest_tick: I24::from(init_tick)
        },
    };

    pool.save();
}