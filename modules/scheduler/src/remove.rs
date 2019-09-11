use super::*;

// This function removes a scheduled event from the List. Note that there will only be one
// scheduled event per contract at any given time.
impl<T: Trait> Module<T> {
    pub fn remove(contract_id: H256) -> Result {
        // Get the current Scheduler List from storage.
        let mut list = <List<T>>::get();

        // Search for the scheduled event with the desired contract_id and
        // then removes it from the list.
        for i in 0..list.len() {
            if list[i].contract_id == contract_id {
                list.swap_remove(i);
                break;
            }
        }

        // Put the Scheduler List into storage and decrease the Scheduler Counter.
        <List<T>>::put(list);
        <Counter<T>>::mutate(|n| *n -= 1);

        // Return Ok.
        Ok(())
    }
}
