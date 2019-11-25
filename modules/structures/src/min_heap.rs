// Copyright 2019 by Trinkler Software AG (Switzerland).
// This file is part of the Katal Chain.
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

//! # Binary Heap
//!
//! Implements a binary min heap using only Rust's Vec. This is necessary because parity_codec doesn't support Rust's BinaryHeap, forcing us to create our own binary heap
//! implementation using parity_codec supported types.
//!
//! The implementation tries to follow Rust's BinaryHeap (https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html), with a few important differences:
//! 1. Rust's BinaryHeap is a max heap, we instead implement a min heap.
//! 2. We only implement a subset of Rust's BinaryHeap methods, namely _new_, _peek_, _push_ and _pop_.
//! 3. Our implementation is compatible with parity_codec.

use super::*;

/// This struct implements the binary min heap. It is a tuple containing a single Vec.
#[derive(Clone, Decode, Debug, Encode, Default, PartialEq)]
pub struct MinHeap<T>(pub Vec<T>)
where
    T: Ord;

impl<T> MinHeap<T>
where
    T: Ord,
{
    /// Creates an empty binary min heap.
    pub fn new() -> MinHeap<T> {
        MinHeap(Vec::new())
    }

    /// Returns an Option of a reference to the smallest item in the binary heap. Returns
    /// None if the heap is empty.
    pub fn peek(&self) -> Option<&T> {
        if self.0.is_empty() {
            return None;
        } else {
            return Some(&self.0[0]);
        }
    }

    /// Pushes an item onto the binary min heap.
    pub fn push(&mut self, x: T) {
        // Push x to the heap. This command will never panic because of the above check.
        self.0.push(x);

        // Get the current number of nodes.
        let mut i = self.0.len() - 1;

        // Performs a "bubble-up" operation.
        while i > 0 && self.0[i] < self.0[(i - 1) / 2] {
            self.0.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    /// Removes the smallest item (the root) from the binary heap and returns an Option
    /// of it. Returns None if the heap is empty.
    pub fn pop(&mut self) -> Option<T> {
        // If the heap is empty, it returns None.
        if self.0.len() == 0 {
            return None;
        }

        // Swaps the top element with the bottom one, then removes it and
        // stores it as an Option.
        let root = Some(self.0.swap_remove(0));

        let mut i: usize = 0;
        let mut t = i;

        // Performs a "bubble-down" operation.
        loop {
            // Checks if there is a left child, and if yes, checks if the child is smaller.
            if i.saturating_mul(2).saturating_add(1) < self.0.len() && self.0[t] > self.0[2 * i + 1]
            {
                t = 2 * i + 1;
            }
            // Checks if there is a right child, and if yes, checks if the child is smaller.
            if i.saturating_mul(2).saturating_add(2) < self.0.len() && self.0[t] > self.0[2 * i + 2]
            {
                t = 2 * i + 2;
            }

            // If no smaller child was found, the loop ends.
            if t == i {
                break;
            // Otherwise, swap places with the smallest child and continue.
            } else {
                self.0.swap(i, t);
                i = t;
            }
        }
        // Returns the previous root of the heap.
        return root;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn new_works() {
        let heap = MinHeap::<u32>::new();
        assert_eq!(heap, MinHeap(Vec::new()));
    }

    #[test]
    fn peek_works() {
        let mut heap = MinHeap(Vec::new());
        assert_eq!(heap.peek(), None);
        heap.push(1);
        assert_eq!(heap.peek(), Some(&1));
    }

    #[test]
    fn push_works() {
        let mut heap = MinHeap(Vec::new());
        let vec = vec![1, 2, 4, 5, 3];
        heap.push(5);
        heap.push(4);
        heap.push(3);
        heap.push(2);
        heap.push(1);
        assert_eq!(heap, MinHeap(vec));
    }

    #[test]
    fn pop_works() {
        let vec = vec![1, 2, 4, 5, 3];
        let mut heap = MinHeap(vec);
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn min_heap_fuzzer() {
        let mut heap = MinHeap::new();
        let mut vec = Vec::new();
        let mut x: u32;

        for i in 0..1000 {
            x = random();
            heap.push(x);
            vec.push(x);
            vec.sort_unstable();
            assert_eq!(heap.peek().unwrap(), &vec[0]);
        }

        for i in 0..1000 {
            assert_eq!(heap.pop().unwrap(), vec[i]);
        }
    }
}
