use chrono::{NaiveTime, TimeDelta};
use rand::Rng;
use std::ops::Add;

use crate::barber_shop::state::BarberShopState;
use crate::barber_shop::BarberShiftState;
use crate::events::customer::{MAX_HAIRCUT_DURATION_MINUTES, MIN_HAIRCUT_DURATION_MINUTES};
use crate::events::shop::SHOP_NAME;
use crate::events::{log, Event, EventEnvelope};

#[derive(Debug)]
pub struct HaircutComplete {
    pub chair: usize,
}

impl Event for HaircutComplete {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        let (customer_id, barber, next_customer_id, next_barber, lights_off) =
            state.complete_haircut(self.chair);

        log(
            &time,
            &format!(
                "{} finished cutting Customer-{}'s hair",
                barber.name, customer_id
            ),
        );

        let active_barber_name = if barber.shift_state == BarberShiftState::FinishingUp {
            log(&time, &format!("{} ended shift", barber.name.clone()));
            next_barber.map(|b| b.name).unwrap_or_default()
        } else {
            barber.name
        };

        let mut events = Vec::new();

        if let Some(next_customer_id) = next_customer_id {
            log(
                &time,
                &format!(
                    "{} started cutting Customer-{}'s hair",
                    active_barber_name, next_customer_id
                ),
            );
            let haircut_duration_minutes = rand::thread_rng()
                .gen_range(MIN_HAIRCUT_DURATION_MINUTES..=MAX_HAIRCUT_DURATION_MINUTES);

            events.push(EventEnvelope::new(
                time.add(TimeDelta::minutes(haircut_duration_minutes)),
                Box::new(HaircutComplete { chair: self.chair }),
            ));
        }

        if lights_off {
            log(&time, &format!("{} is closed", SHOP_NAME));
        }

        (state, events)
    }
}
