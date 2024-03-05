use chrono::NaiveTime;

use crate::barber_shop::state::BarberShopState;
use crate::events::customer::CustomerKickedOut;
use crate::events::{log, Event, EventEnvelope};

pub static SHOP_NAME: &str = "Chopper's Clippers";

#[derive(Debug)]
pub struct ShopOpen;

impl Event for ShopOpen {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        state.open_shop();

        log(&time, &format!("{} is open for business!", SHOP_NAME));

        (state, vec![])
    }
}

#[derive(Debug)]
pub struct ShopClose;

impl Event for ShopClose {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        let waiting_customers = state.close_shop();

        let events = waiting_customers
            .into_iter()
            .map(|customer| EventEnvelope::new(time, Box::new(CustomerKickedOut { id: customer })))
            .collect();

        (state, events)
    }
}
