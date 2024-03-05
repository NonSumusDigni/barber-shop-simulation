use derive_more::{Display, From, Into};

pub mod state;

#[derive(Debug, Default)]
pub struct Chair {
    barber: Option<BarberId>,
    customer: Option<CustomerId>,
}

#[derive(Clone, Copy, Debug, Display, Eq, From, Into, Hash, PartialEq)]
pub struct CustomerId(u32);

#[derive(Clone, Debug)]
pub struct Barber {
    pub id: BarberId,
    pub name: String,
    pub shift_state: BarberShiftState,
}

#[derive(Clone, Copy, Debug, Eq, From, Into, Hash, PartialEq)]
pub struct BarberId(usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BarberShiftState {
    WaitingForChair,
    Working,
    FinishingUp,
}

pub enum CustomerArrivalResult {
    Serviced(Haircut),
    Waiting,
    NoRoom,
    Closed(ClosingState),
}

pub struct Haircut {
    pub barber_name: String,
    pub chair: usize,
}

pub enum ClosingState {
    LightsOn,
    LightsOff,
}
