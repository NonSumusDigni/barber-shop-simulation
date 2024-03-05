use std::collections::{HashMap, VecDeque};
use std::ops::IndexMut;

use super::{
    Barber, BarberId, BarberShiftState, Chair, ClosingState, CustomerArrivalResult, CustomerId,
    Haircut,
};

static NUM_CHAIRS: u32 = 4;
static WAITING_ROOM_CAPACITY: usize = 4;

// make sure to add enough names if you add more than 8 barbers, lest the program will panic
static BARBER_NAMES: [&str; 8] = [
    "Pat", "Kara", "Vanessa", "Arnold", "Kelly", "Russ", "Sidney", "Saul",
];

#[derive(Debug)]
pub struct BarberShopState {
    barbers: HashMap<BarberId, Barber>,

    chairs: Vec<Chair>,
    customers_waiting: VecDeque<CustomerId>,
    barbers_waiting: VecDeque<BarberId>,

    open: bool,
    customer_count: u32,
}

impl BarberShopState {
    pub fn new() -> BarberShopState {
        let barbers = HashMap::new();
        let chairs = (0..NUM_CHAIRS).map(|_| Chair::default()).collect();
        let customers_waiting = VecDeque::new();
        let barbers_waiting = VecDeque::new();

        BarberShopState {
            barbers,
            chairs,
            customers_waiting,
            barbers_waiting,
            open: false,
            customer_count: 0,
        }
    }

    pub fn add_barber(&mut self) -> Barber {
        let id = self.barbers.len().into();

        let shift_state = match self.chairs.iter().position(|c| c.barber.is_none()) {
            Some(i) => {
                self.chairs[i].barber = Some(id);
                BarberShiftState::Working
            }
            None => {
                self.barbers_waiting.push_back(id);
                BarberShiftState::WaitingForChair
            }
        };

        let name = BARBER_NAMES[usize::from(id)].to_string();
        let barber = Barber {
            id,
            name,
            shift_state,
        };

        self.barbers.insert(id, barber.clone());

        barber
    }

    pub fn open_shop(&mut self) {
        self.open = true;
    }

    pub fn close_shop(&mut self) -> Vec<CustomerId> {
        self.open = false;
        self.customers_waiting.drain(..).collect()
    }

    pub fn add_customer(&mut self) -> (CustomerId, CustomerArrivalResult) {
        let id = self.customer_count.into();
        self.customer_count += 1;

        // are we open?
        if !self.open {
            // no, but are there still haircuts being finished?
            let closing_state = if self.chairs.iter().any(|c| c.customer.is_some()) {
                ClosingState::LightsOn
            } else {
                ClosingState::LightsOff
            };

            return (id, CustomerArrivalResult::Closed(closing_state));
        }

        // is there an open chair?
        for i in 0..self.chairs.len() {
            let chair = self.chairs.index_mut(i);
            if chair.customer.is_none() {
                chair.customer = Some(id);

                let barber_id = chair.barber.expect("barberless chair");
                let barber = self.barbers.get(&barber_id).expect("barber not found");
                let barber_name = barber.name.clone();

                return (
                    id,
                    CustomerArrivalResult::Serviced(Haircut {
                        barber_name,
                        chair: i,
                    }),
                );
            }
        }

        // no open chairs, is there room in the waiting room?
        if self.customers_waiting.len() < WAITING_ROOM_CAPACITY {
            self.customers_waiting.push_back(id);
            (id, CustomerArrivalResult::Waiting)
        } else {
            (id, CustomerArrivalResult::NoRoom)
        }
    }

    pub fn customer_wait_expired(&mut self, customer_id: CustomerId) -> CustomerWaitOutcome {
        match self
            .customers_waiting
            .iter()
            .position(|&id| id == customer_id)
        {
            None => CustomerWaitOutcome::Serviced,
            Some(index) => {
                self.customers_waiting.remove(index);
                CustomerWaitOutcome::Leaves
            }
        }
    }

    pub fn barber_shift_ends(&mut self, barber_id: BarberId) {
        let barber = self.barbers.get_mut(&barber_id).expect("barber not found");
        barber.shift_state = BarberShiftState::FinishingUp;
    }

    pub fn complete_haircut(
        &mut self,
        chair_id: usize,
    ) -> (CustomerId, Barber, Option<CustomerId>, Option<Barber>, bool) {
        let chair = self.chairs.index_mut(chair_id);
        let barber_id = chair.barber.take().expect("barberless chair");
        let barber = self
            .barbers
            .get(&barber_id)
            .expect("barber not found")
            .clone();
        let customer_id = chair.customer.take().expect("customerless haircut");

        let mut next_barber_cloned = None;
        // check if this was the current barber's last haircut
        if barber.shift_state == BarberShiftState::FinishingUp {
            if let Some(next_barber_id) = self.barbers_waiting.pop_front() {
                chair.barber = Some(next_barber_id);
                let next_barber = self
                    .barbers
                    .get_mut(&next_barber_id)
                    .expect("barber not found");
                next_barber.shift_state = BarberShiftState::Working;
                next_barber_cloned = Some(next_barber.clone());
            }
        } else {
            chair.barber = Some(barber_id);
        }

        // now check if there is a customer waiting
        if let Some(next_customer_id) = self.customers_waiting.pop_front() {
            chair.customer = Some(next_customer_id);
        }

        (
            customer_id,
            barber,
            chair.customer,
            next_barber_cloned,
            self.lights_off(),
        )
    }

    fn lights_off(&self) -> bool {
        !self.open
            && self.customers_waiting.is_empty()
            && self.chairs.iter().all(|c| c.customer.is_none())
    }
}

pub enum CustomerWaitOutcome {
    Serviced,
    Leaves,
}
