#![feature(box_syntax)]

pub mod dpqueue {
    use std::cmp::Ordering;
    use std::default::Default;

    pub struct DualPriorityQueueItem<T, P1, P2> {
        pub item: Box<T>,
        pub p1: P1,
        pub p2: P2
    }

    pub struct DualPriorityQueue<T, P1, P2> {
        pub items: Vec<DualPriorityQueueItem<T, P1, P2>>,
    }

    impl<T, P1, P2> DualPriorityQueue<T, P1, P2>
        where P1: Copy + Default + Ord,
              P2: Copy + Default + Ord
    {
        pub fn new() -> DualPriorityQueue<T, P1, P2> {
            DualPriorityQueue { items: Vec::new() }
        }

        pub fn enqueue(&mut self, item: T, p1: P1, p2: P2) {
            self.items.push(DualPriorityQueueItem {
                item: box item,
                p1: p1,
                p2: p2,
            });
        }

        pub fn dequeue_p1(&mut self) -> Option<T> { self.dequeue(|item| item.p1) }
        pub fn dequeue_p2(&mut self) -> Option<T> { self.dequeue(|item| item.p2) }

        fn dequeue<P, F: Fn(&DualPriorityQueueItem<T, P1, P2>) -> P>(&mut self, f: F) -> Option<T>
            where P: Copy + Default + Ord
        {
            let (_, max_idx, _) = self.items.iter().fold((0, 0, Default::default()), |(idx, max_idx, priority), dpq_item| {
                match f(dpq_item).cmp(&priority) {
                    Ordering::Greater => (idx + 1, idx, f(dpq_item)),
                    _                 => (idx + 1, max_idx, priority),
                }
            });

            if self.items.len() > max_idx {
                Some(*self.items.swap_remove(max_idx).item)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
pub mod dpqueue_tests {
    use super::dpqueue::DualPriorityQueue;

    #[test]
    fn can_enqueue_items() {
        let mut queue = DualPriorityQueue::new();

        queue.enqueue("Testing!", 5, 6);
        queue.enqueue("1, 2, 3.", 6, 5);

        assert!(*queue.items[0].item == "Testing!");
        assert!(*queue.items[1].item == "1, 2, 3.")
    }

    #[test]
    fn can_dequeue_items() {
        let mut queue = DualPriorityQueue::new();

        queue.enqueue("Testing!", 5, 1);
        queue.enqueue("1, 2, 3.", 1, 5);
        queue.enqueue("Bob.", 6, 1);

        assert!(queue.dequeue_p1().unwrap() == "Bob.");
        assert!(queue.dequeue_p2().unwrap() == "1, 2, 3.");
        assert!(queue.dequeue_p1().unwrap() == "Testing!");
    }
}
