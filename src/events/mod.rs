pub mod barber;
pub mod customer;
pub mod haircut;
pub mod shop;

use std::cmp::Ordering;
use std::fmt::Debug;

use chrono::NaiveTime;

use crate::barber_shop::state::BarberShopState;

static TIME_FORMAT: &str = "%H:%M";

pub fn log(time: &NaiveTime, message: &str) {
    let formatted_time = format_time(time);
    println!("[{formatted_time}] {message}");
}

fn format_time(time: &NaiveTime) -> String {
    time.format(TIME_FORMAT).to_string()
}

#[derive(Debug)]
pub struct EventEnvelope {
    pub time: NaiveTime,
    event: Box<dyn Event>,
}

impl EventEnvelope {
    pub fn new(time: NaiveTime, event: Box<dyn Event>) -> EventEnvelope {
        EventEnvelope { time, event }
    }

    pub fn apply(self, state: BarberShopState) -> (BarberShopState, Vec<EventEnvelope>) {
        self.event.apply(state, self.time)
    }
}

impl PartialEq for EventEnvelope {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for EventEnvelope {}

impl PartialOrd for EventEnvelope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventEnvelope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

pub trait Event: Debug {
    fn apply(
        &self,
        state: BarberShopState,
        time: NaiveTime,
    ) -> (BarberShopState, Vec<EventEnvelope>);
}
