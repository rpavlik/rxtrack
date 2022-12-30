// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

/// Person ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonId(usize);

/// Person info
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonInfo {
    person_id: PersonId,
    person_name: String,
}

/// Prescription ID
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RxId(usize);

pub struct RxInfo {
    rx_id: RxId,
    person_id: PersonId,
    rx_name: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RefillPolicyTiming {
    /// May fill a given number of days after the most recent fill
    SincePreviousFill(usize),
    /// May pick up a given number of days after the most recent pickup
    SincePreviousPickup(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct RefillPolicy {
    timing: RefillPolicyTiming,
    needs_new_rx_each_time: bool,
}

impl RefillPolicy {
    pub fn new(timing: RefillPolicyTiming, needs_new_rx_each_time: bool) -> Self {
        Self {
            timing,
            needs_new_rx_each_time,
        }
    }
}

mod entities;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
