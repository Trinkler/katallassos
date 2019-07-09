//
// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of Katal.
//
// Katal is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

use super::*;

/// The contract event struct, it is composed of a time and an event type. It can be ordered, first
/// by time (from earliest to latest) and secondly by priority of event type (from highest to lowest).
#[derive(Clone, Copy, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ContractEvent {
    pub time: Time,
    pub event_type: ContractEventType,
}

/// All ACTUS contract event types as specifed in the ACTUS paper. They are ordered from highest to
/// lowest priority.
#[derive(Clone, Copy, Decode, Encode, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum ContractEventType {
    IED,
    IPCI,
    IP,
    FP,
    PR,
    PI,
    PRF,
    PY,
    PP,
    CD,
    RRF,
    RR,
    DV,
    PRD,
    IMP,
    MP,
    TD,
    SC,
    IPCB,
    XD,
    STD,
    MD,
    // AD, // This event is for analysis only, it has no effect on the state.
}

impl ContractEvent {
    /// A constructor for the ContractEvent type. It is just syntactic sugar.
    pub fn new(time: Time, event_type: ContractEventType) -> ContractEvent {
        ContractEvent {
            time: time,
            event_type: event_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_sorted() {
        let t1 = Time::from_values(2016, 11, 8, 19, 00, 00);
        let t2 = Time::from_values(2017, 1, 20, 12, 00, 00);
        let e1 = ContractEventType::PR;
        let e2 = ContractEventType::CD;

        let x1 = ContractEvent::new(t1, e1);
        let x2 = ContractEvent::new(t1, e2);
        let x3 = ContractEvent::new(t2, e1);
        let x4 = ContractEvent::new(t2, e2);

        let mut v1 = vec![x4, x2, x1, x3];
        v1.sort_unstable();
        let v2 = vec![x1, x2, x3, x4];

        assert_eq!(v1, v2);
    }
}
