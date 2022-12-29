// Copyright 2022, Ryan Pavlik <ryan@ryanpavlik.com>
// SPDX-License-Identifier: GPL3+

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
