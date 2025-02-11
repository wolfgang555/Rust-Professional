use std::cmp::Ord;
use std::default::Default;
use std::mem;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        // Increment count and push the new value
        self.count += 1;

        // If the items vector is too small, extend it
        if self.count >= self.items.len() {
            self.items.push(value);
        } else {
            self.items[self.count] = value;
        }

        // Perform sift up (bubble up) operation
        let mut current_idx = self.count;
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx);

            // Compare using the custom comparator
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                // Swap if the current item should be higher in the heap
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // If no right child, return left child
        if right_idx > self.count {
            return left_idx;
        }

        // Use the comparator to determine which child is "smaller"
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // If the heap is empty, return None
        if self.is_empty() {
            return None;
        }

        // Take the top item (first valid index)
        let result = mem::take(&mut self.items[1]);

        // Replace the top item with the last item
        self.items[1] = mem::take(&mut self.items[self.count]);
        self.count -= 1;

        // Perform sift down (bubble down) operation
        let mut current_idx = 1;
        while self.children_present(current_idx) {
            let child_idx = self.smallest_child_idx(current_idx);

            // Compare using the custom comparator
            if (self.comparator)(&self.items[child_idx], &self.items[current_idx]) {
                // Swap if the child should be higher in the heap
                self.items.swap(current_idx, child_idx);
                current_idx = child_idx;
            } else {
                break;
            }
        }

        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}