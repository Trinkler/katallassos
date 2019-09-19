use super::*;

// This struct defines a scheduled event.
#[derive(Copy, Clone, Decode, Encode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ScheduledEvent {
    pub time: Time,
    pub contract_id: H256,
    pub index: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for ScheduledEvent {
    fn cmp(&self, other: &ScheduledEvent) -> Ordering {
        // Notice that the we flip the ordering on time.
        // In case of a tie we compare contract_id and index - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .time
            .cmp(&self.time)
            .then_with(|| self.contract_id.cmp(&other.contract_id))
            .then_with(|| self.index.cmp(&other.index))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &ScheduledEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

        let mut heap = BinaryHeap::new();
        heap.push(d);
        heap.push(b);
        heap.push(a);
        heap.push(c);

        assert_eq!(heap.pop(), Some(a));
        assert_eq!(heap.pop(), Some(b));
        assert_eq!(heap.pop(), Some(c));
        assert_eq!(heap.pop(), Some(d));
        assert_eq!(heap.pop(), None);
    }
}
