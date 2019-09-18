// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as SchedulerStorage {
        // TODO: Substitute this Vector with a better data structure. Desired properties are:
        //      - O(log n) insertions
        //      - O(log n) deletions
        //      - Keeps being sorted after each insertion and deletion
        pub List: Vec<ScheduledEvent> = Vec::new();
        pub Counter: u32 = 0;
    }
}
