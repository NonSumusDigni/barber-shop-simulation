use crate::barber_shop::BarberId;
use chrono::{NaiveTime, TimeDelta};
use std::ops::Add;

use crate::barber_shop::state::BarberShopState;

use super::{log, Event, EventEnvelope};

static BARBER_SHIFT_DURATION_HOURS: i64 = 4;

#[derive(Debug)]
pub struct BarberShiftStart;

impl Event for BarberShiftStart {
    fn apply(
        &self,
        mut state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        let barber = state.add_barber();

        log(&time, &format!("{} started shift", barber.name));

        // schedule the end of their shift
        let events = vec![EventEnvelope {
            time: time.add(TimeDelta::hours(BARBER_SHIFT_DURATION_HOURS)),
            event: Box::new(BarberShiftEnd { id: barber.id }),
        }];

        (state, events)
    }
}

#[derive(Debug)]
pub struct BarberShiftEnd {
    pub id: BarberId,
}

impl Event for BarberShiftEnd {
    fn apply(
        &self,
        mut state: BarberShopState,
        _time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>) {
        state.barber_shift_ends(self.id);
        (state, vec![])
    }
}
