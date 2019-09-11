use super::*;

// This function adds a scheduled event to the List. Note that there will only be one
// scheduled event per contract at any given time.
impl<T: Trait> Module<T> {
    pub fn add(contract_id: H256, time: Time, index: u32) -> Result {
        // This check is necessary also because the number of elements in a vector
        // cannot exceed 'usize', which is at least 32 bits.
        if <Counter<T>>::get() == u32::max_value() {
            return Err("Scheduler list is full");
        }

        // Create the new event.
        let event = ScheduledEvent {
            contract_id: contract_id,
            time: time,
            index: index,
        };
        // Get the current Scheduler List from storage.
        let mut list = <List<T>>::get();
        // Add the event to the Scheduler List.
        list.push(event);
        // Put the Scheduler List into storage and increase the Scheduler Counter.
        <List<T>>::put(list);
        <Counter<T>>::mutate(|n| *n += 1);

        // Return Ok.
        Ok(())
    }
}
