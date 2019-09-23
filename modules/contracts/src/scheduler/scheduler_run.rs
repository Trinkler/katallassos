use super::*;

// This function executes all events for which the time has come.
impl<T: Trait> Module<T> {
    pub fn scheduler_run(now: Time) -> Result {
        // Get the current Scheduler Heap from storage.
        let mut heap = <Scheduler<T>>::get();

        // This loop goes through every scheduled event that is smaller than the
        // current time.
        while heap.peek().is_some() && now >= heap.peek().unwrap().time {
            let mut scheduled_event = heap.pop().unwrap();

            // Get the state of the ACTUS contract and the corresponding
            // contract event type to be executed.
            let mut state = <Contracts<T>>::get(scheduled_event.contract_id);
            let event = state.schedule[scheduled_event.index as usize];

            // Make the ACTUS contract progress.
            <Module<T>>::progress(event, state.clone())?;

            // This loop executes the remaining events of the current contract for which
            // the time has come. This is more efficient than just pushing the next event
            // to the Scheduler heap.
            scheduled_event.index += 1;
            while scheduled_event.index < state.schedule.len() as u32 {
                // Get the next event for this contract.
                let event = state.schedule[scheduled_event.index as usize];
                // Compare the event's time with the current time.
                if now >= event.time {
                    // Make the ACTUS contract progress.
                    <Module<T>>::progress(event, state.clone())?;
                    // Increment the index.
                    scheduled_event.index += 1;
                } else {
                    // Update the time for the scheduled event and push it to the heap.
                    scheduled_event.time = event.time;
                    heap.push(scheduled_event);
                }
            }
        }

        // Put the Scheduler Heap into storage.
        <Scheduler<T>>::put(heap);

        // Return Ok.
        Ok(())
    }
}
