extern crate alloc;
use fuel_indexer_macros::{graphql_schema, handler};
use fuels_abigen_macro::wasm_abigen;

graphql_schema!("concentrated_liquidity", "schema/schema.graphql");
wasm_abigen!(
    no_name,
    "hello-indexer/contracts/hello_indexer.json"
);

#[handler]
fn handle_init(event: SwapEvent) {
    Logger::info("Callin' the event handler");
    let LogEvent {
        contract,
        event_id,
        count,
    } = event;

    let mut t1 = match EventCount::load(event_id) {
        Some(t) => t,
        None => EventCount {
            id: event_id,
            account: Address::from(contract),
            count: 0,
        },
    };

    t1.count += count;

    t1.save();
}

#[handler]
fn handle_swap(event: SwapEvent) {
    Logger::info("Callin' the event handler");
    let LogEvent {
        contract,
        event_id,
        count,
    } = event;

    let mut t1 = match EventCount::load(event_id) {
        Some(t) => t,
        None => EventCount {
            id: event_id,
            account: Address::from(contract),
            count: 0,
        },
    };

    t1.count += count;

    t1.save();
}

#[handler]
fn handle_mint(event: SwapEvent) {
    Logger::info("Callin' the event handler");
    let LogEvent {
        contract,
        event_id,
        count,
    } = event;

    let mut t1 = match EventCount::load(event_id) {
        Some(t) => t,
        None => EventCount {
            id: event_id,
            account: Address::from(contract),
            count: 0,
        },
    };

    t1.count += count;

    t1.save();
}

#[handler]
fn handle_burn(event: SwapEvent) {
    Logger::info("Callin' the event handler");
    let LogEvent {
        contract,
        event_id,
        count,
    } = event;

    let mut t1 = match EventCount::load(event_id) {
        Some(t) => t,
        None => EventCount {
            id: event_id,
            account: Address::from(contract),
            count: 0,
        },
    };

    t1.count += count;

    t1.save();
}