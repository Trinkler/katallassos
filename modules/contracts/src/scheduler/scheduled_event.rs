use super::*;

// This struct defines a scheduled event.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScheduledEvent {
    pub time: Time,
    pub contract_id: H256,
    pub index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_heap_is_min_heap() {
        let a = ScheduledEvent {
            time: Time::from_values(1969, 07, 16, 13, 32, 00),
            contract_id: H256::random(),
            index: 0,
        };
        let b = ScheduledEvent {
            time: Time::from_values(1969, 07, 20, 20, 17, 00),
            contract_id: H256::random(),
            index: 0,
        };
        let c = ScheduledEvent {
            time: Time::from_values(1969, 07, 21, 02, 56, 15),
            contract_id: H256::random(),
            index: 0,
        };
        let d = ScheduledEvent {
            time: Time::from_values(1969, 07, 24, 16, 51, 00),
            contract_id: H256::random(),
            index: 0,
        };

        let mut heap = MinHeap::new();
        heap.push(d);
        heap.push(b);
        heap.push(a);
        heap.push(c);

        assert!(a < b);
        assert!(b < c);
        assert!(c < d);

        assert_eq!(heap.pop(), Some(a));
        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), Some(d));
        assert_eq!(heap.pop(), None);
    }
}
