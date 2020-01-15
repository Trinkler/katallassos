// Copyright 2020 by Trinkler Software AG (Switzerland).
// This file is part of Katal Chain.
//
// Katal Chain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version <http://www.gnu.org/licenses/>.
//
// Katal Chain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use super::*;

/// The contract event struct, it is composed of a time and an event type. It can be ordered, first
/// by time (from earliest to latest) and secondly by priority of event type (from highest to lowest).
#[derive(Clone, Copy, Decode, Debug, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub time: Time,
    pub event_type: EventType,
}

/// All ACTUS contract event types as specifed in the ACTUS paper. They are ordered from highest to
/// lowest priority.
#[derive(Clone, Copy, Decode, Debug, Encode, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventType {
    IED,
    FP,
    PR,
    PD,
    PRF,
    PY,
    PP,
    IP,
    IPCI,
    CE,
    RRF,
    RR,
    DV,
    PRD,
    MR,
    TD,
    SC,
    IPCB,
    MD,
    XD,
    STD,
    // AD, // This event is for analysis only, it has no effect on the state.
}

impl Event {
    /// A constructor for the Event type. It is just syntactic sugar.
    pub fn new(time: Time, event_type: EventType) -> Event {
        Event {
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
        let e1 = EventType::PR;
        let e2 = EventType::CE;

        let x1 = Event::new(t1, e1);
        let x2 = Event::new(t1, e2);
        let x3 = Event::new(t2, e1);
        let x4 = Event::new(t2, e2);

        let mut v1 = vec![x4, x2, x1, x3];
        v1.sort_unstable();
        let v2 = vec![x1, x2, x3, x4];

        assert_eq!(v1, v2);
    }
}
