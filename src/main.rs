mod barber_shop;
mod events;

use std::ops::Add;

use chrono::{NaiveTime, TimeDelta};
use sorted_vec::SortedVec;

use barber_shop::state::BarberShopState;
use events::barber::BarberShiftStart;
use events::customer::CustomerArrival;
use events::shop::{ShopClose, ShopOpen};
use events::EventEnvelope;

static OPENING_TIME_HOUR: u32 = 9;
static CLOSING_TIME_HOUR: u32 = 17;

fn main() {
    let mut state = BarberShopState::new();
    let (mut events, mut envelope_id) = bootstrap_events();

    while !events.is_empty() {
        // println!("\nSTATE\n===\n{:?}\n\n{:?}\n====\n", state, events);
        let event = events.remove_index(0);
        // println!("PROCESSING EVENT: {:?}", event.inner);
        let (new_state, new_events) = event.inner.apply(state);
        state = new_state;
        for event in new_events {
            events.insert(WrappedEventEnvelope::new(event, envelope_id));
            envelope_id += 1;
        }
    }
}

fn bootstrap_events() -> (SortedVec<WrappedEventEnvelope>, u64) {
    let mut events = SortedVec::new();
    let mut envelope_id = 0;

    // shop open and close
    let shop_open_time = NaiveTime::from_hms_opt(OPENING_TIME_HOUR, 0, 0).expect("opening time");
    let shop_close_time = NaiveTime::from_hms_opt(CLOSING_TIME_HOUR, 0, 0).expect("closing time");

    events.insert(WrappedEventEnvelope::new(
        EventEnvelope::new(shop_open_time, Box::new(ShopOpen)),
        envelope_id,
    ));
    envelope_id += 1;

    events.insert(WrappedEventEnvelope::new(
        EventEnvelope::new(shop_close_time, Box::new(ShopClose)),
        envelope_id,
    ));
    envelope_id += 1;

    // barber shift starts
    let shift_1_start = shop_open_time;
    let shift_2_start = shop_open_time.add(TimeDelta::hours(4));
    for _ in 0..4 {
        events.insert(WrappedEventEnvelope::new(
            EventEnvelope::new(shift_1_start, Box::new(BarberShiftStart)),
            envelope_id,
        ));
        envelope_id += 1;
    }
    for _ in 0..4 {
        events.insert(WrappedEventEnvelope::new(
            EventEnvelope::new(shift_2_start, Box::new(BarberShiftStart)),
            envelope_id,
        ));
        envelope_id += 1;
    }

    // first customer arrival
    events.insert(WrappedEventEnvelope::new(
        EventEnvelope::new(shop_open_time, Box::new(CustomerArrival)),
        envelope_id,
    ));
    envelope_id += 1;

    (events, envelope_id)
}

// this is a bit of a hack, to make the SortedVec remember the order of insertion as a tie-breaker
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WrappedEventEnvelope {
    pub inner: EventEnvelope,
    id: u64,
}

impl WrappedEventEnvelope {
    pub fn new(inner: EventEnvelope, id: u64) -> Self {
        Self { inner, id }
    }
}
