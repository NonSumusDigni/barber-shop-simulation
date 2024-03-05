use std::ops::Add;

use chrono::{NaiveTime, TimeDelta};
use rand::Rng;

use crate::barber_shop::state::{BarberShopState, CustomerWaitOutcome};
use crate::barber_shop::{ClosingState, CustomerArrivalResult, CustomerId};
use crate::events::haircut::HaircutComplete;
use crate::events::{log, Event, EventEnvelope};

static CUSTOMER_ARRIVAL_INTERVAL_MINUTES: i64 = 5;
pub static MIN_HAIRCUT_DURATION_MINUTES: i64 = 20;
pub static MAX_HAIRCUT_DURATION_MINUTES: i64 = 40;
static MAX_WAIT_DURATION_MINUTES: i64 = 20;

#[derive(Debug)]
pub struct CustomerArrival;

impl Event for CustomerArrival {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        let (customer_id, result) = state.add_customer();

        let mut events = Vec::new();

        log(&time, &format!("Customer-{} entered", customer_id));

        match &result {
            CustomerArrivalResult::Serviced(haircut) => {
                log(
                    &time,
                    &format!(
                        "{} started cutting Customer-{}'s hair",
                        haircut.barber_name, customer_id
                    ),
                );

                let haircut_duration_minutes = rand::thread_rng()
                    .gen_range(MIN_HAIRCUT_DURATION_MINUTES..=MAX_HAIRCUT_DURATION_MINUTES);

                events.push(EventEnvelope::new(
                    time.add(TimeDelta::minutes(haircut_duration_minutes)),
                    Box::new(HaircutComplete {
                        chair: haircut.chair,
                    }),
                ));
            }
            CustomerArrivalResult::Waiting => {
                events.push(EventEnvelope::new(
                    time.add(TimeDelta::minutes(MAX_WAIT_DURATION_MINUTES)),
                    Box::new(CustomerWaitExpired { id: customer_id }),
                ));
            }
            CustomerArrivalResult::NoRoom => {
                log(
                    &time,
                    &format!("Customer-{} leaves unfulfilled", customer_id),
                );
            }
            CustomerArrivalResult::Closed(_) => {
                log(
                    &time,
                    &format!("Customer-{} leaves disappointed", customer_id),
                );
            }
        }

        if !matches!(
            &result,
            CustomerArrivalResult::Closed(ClosingState::LightsOn)
        ) {
            // schedule the next customer arrival
            events.push(EventEnvelope::new(
                time.add(TimeDelta::minutes(CUSTOMER_ARRIVAL_INTERVAL_MINUTES)),
                Box::new(CustomerArrival),
            ));
        }

        (state, events)
    }
}

#[derive(Debug)]
pub struct CustomerWaitExpired {
    pub id: CustomerId,
}

impl Event for CustomerWaitExpired {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        // this timer goes off regardless, so only log if the customer was still waiting
        if let CustomerWaitOutcome::Leaves = state.customer_wait_expired(self.id) {
            log(&time, &format!("Customer-{} leaves frustrated", self.id));
        }

        (state, vec![])
    }
}

#[derive(Debug)]
pub struct CustomerKickedOut {
    pub id: CustomerId,
}

impl Event for CustomerKickedOut {
    fn apply(
        &self,
        state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        log(&time, &format!("Customer-{} leaves cursing", self.id));
        (state, vec![])
    }
}
