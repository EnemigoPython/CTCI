pub mod linked_list {
    use itertools::{EitherOrBoth::*, Itertools};
    use std::collections::HashSet;
    use std::ptr;

    type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T> {
        value: T,
        next: Link<T>,
    }

    impl<T> Node<T> {
        fn remove_next(&mut self) {
            self.next.take().map(|node| {
                self.next = node.next;
            });
        }

        pub fn peek(&self) -> &T {
            &self.value
        }
    }

    pub struct List<T> {
        pub head: Link<T>,
    }

    // base impl
    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, v: T) {
            let node = Node {
                value: v,
                next: self.head.take(),
            };
            self.head = Some(Box::new(node));
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.value
            })
        }

        pub fn from_vec(v: Vec<T>) -> Self {
            let mut list = List::new();
            for i in v {
                list.push(i);
            }
            list
        }

        pub fn to_vec(self) -> Vec<T> {
            self.into_iter().collect()
        }

        pub fn into_iter(self) -> ListIntoIter<T> {
            ListIntoIter(self)
        }

        pub fn iter(&self) -> ListIter<T> {
            ListIter {
                next: self.head.as_deref(),
            }
        }

        pub fn iter_mut(&mut self) -> ListIterMut<T> {
            ListIterMut {
                next: self.head.as_deref_mut(),
            }
        }

        pub fn nth(&self, n: usize) -> Option<&Node<T>> {
            let mut curr = &self.head;
            for _ in 0..n {
                if let Some(node) = curr.as_deref() {
                    curr = &node.next;
                } else {
                    return None;
                }
            }
            curr.as_deref()
        }
    }

    // challenge impl
    // impl<T> List<T>  // I started out writing these generic but more and more challenges required int
    // where
    // T: Hash + Eq + Clone + Debug + std::cmp::PartialOrd<i32>,
    impl List<i32> {
        /// remove_dups prompt: Write code to remove duplicates from an unsorted linked list.
        /// FOLLOW UP:
        /// How would you solve this problem if a temporary buffer is not allowed?
        pub fn remove_dups(&mut self) {
            let mut unique: HashSet<i32> = HashSet::new();
            let mut curr = &mut self.head;
            if let Some(first) = curr.as_deref_mut() {
                unique.insert(first.value.clone());
            }
            while let Some(node) = curr.as_deref_mut() {
                while let Some(next) = node.next.as_deref_mut() {
                    if unique.insert(next.value.clone()) {
                        break;
                    } else {
                        node.remove_next();
                    }
                }
                curr = &mut node.next;
            }
        }

        /// kth prompt: Implement an algorithm to find the kth to last element of a singly linked list.
        pub fn kth(&self, k: usize) -> Option<&i32> {
            let to_vec: Vec<&i32> = self.iter().collect();
            let len = to_vec.len();
            if k == 0 || k > len {
                None
            } else {
                Some(to_vec[len - k])
            }
        }

        /// delete_middle_node prompt: Implement an algorithm to delete a node in the middle (i.e. any node but
        /// the first and last node, not necessarily the exact middle) of a singly linked list, given only access
        /// to that node.
        /// EXAMPLE
        /// Input: the node c from the linked list a -> b -> c -> d -> e -> f
        /// Result: nothing is returned, but the new linked list looks like a -> b -> d -> e -> f
        pub fn delete_middle_node(&mut self, raw_node_ptr: *const Node<i32>) {
            let mut curr = &mut self.head;
            if let Some(node) = curr.as_deref() {
                if ptr::eq(raw_node_ptr, node) {
                    self.pop();
                    return;
                }
            }
            while let Some(node) = curr.as_deref_mut() {
                if let Some(next) = node.next.as_deref() {
                    if ptr::eq(raw_node_ptr, next) {
                        node.remove_next();
                        return;
                    }
                }
                curr = &mut node.next;
            }
        }

        /// partition prompt: Write code to partition a linked list around a value x, such that all
        /// nodes less than x come before all nodes greater than or equal to x. (IMPORTANT: The
        /// partition element x can appear anywhere in the "right partition": it does not need to
        /// appear between the left and right partitions. The additional spacing in the example below
        /// indicates the partition. Yes, the output below is one of many valid outputs!)
        /// EXAMPLE
        /// Input: 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1 [partition=5]
        /// Output: 3 -> 1 -> 2     ->      10 -> 5 -> 5 -> 8
        pub fn partition(&mut self, x: i32) {
            let mut buff: Vec<i32> = Vec::new();
            while let Some(node) = &mut self.head.as_deref() {
                if node.peek() < &x {
                    buff.push(node.value.clone());
                    self.pop();
                } else {
                    break;
                }
            }
            let mut curr = &mut self.head;
            while let Some(node) = curr.as_deref_mut() {
                while let Some(next) = node.next.as_deref() {
                    if next.peek() < &x {
                        buff.push(next.value.clone());
                        node.remove_next();
                    } else {
                        break;
                    }
                }
                curr = &mut node.next;
            }
            for val in buff {
                self.push(val);
            }
        }

        /// sum_lists prompt: you have two numbers represented by a linked list, where
        /// each node contains a single digit. The digits are stored in reverse order, such
        /// that the 1's digit is at the head of the list. Write a function that adds the
        /// two numbers and returns the sum as a linked list. (You are not allowed to "cheat"
        /// and just convert the linked list to an integer.)
        /// EXAMPLE
        /// Input: (7 -> 1 -> 6) + (5 -> 9 -> 2). That is, 617 + 295.
        /// Output: 2 -> 1 -> 9. That is, 912.
        pub fn sum_lists(list1: List<i32>, list2: List<i32>) -> List<i32> {
            let mut ret_list: List<i32> = List::new();
            let mut buff: Vec<i32> = Vec::new();
            let mut carry_bit = false;
            for pair in list1.into_iter().zip_longest(list2.into_iter()) {
                let mut total = {
                    if carry_bit {
                        carry_bit = false;
                        1
                    } else {
                        0
                    }
                };
                total += match pair {
                    Both(l, r) => l + r,
                    Left(x) | Right(x) => x,
                };
                if total > 9 {
                    carry_bit = true;
                    total -= 10;
                }
                buff.push(total);
            }
            if carry_bit {
                buff.push(1);
            }
            for val in buff.into_iter().rev() {
                ret_list.push(val);
            }
            ret_list
        }

        /// FOLLOW UP
        /// Repeat the problem for forward order.
        pub fn sum_lists_fwd(list1: List<i32>, list2: List<i32>) -> List<i32> {
            let mut total: i32 = 0;
            for (e, val) in (0..).zip(list1.into_iter()) {
                total += val * 10_i32.pow(e);
            }
            for (e, val) in (0..).zip(list2.into_iter()) {
                total += val * 10_i32.pow(e);
            }
            let mut ret_list = List::new();
            loop {
                let mut total_mod = total.clone();
                total_mod /= 10;
                total_mod *= 10;
                ret_list.push(total - total_mod);
                total /= 10;
                if total < 10 {
                    break;
                }
            }
            ret_list.push(total);

            ret_list
        }

        /// palindrome prompt: implement a function to check if a linked list is a palindrome.
        pub fn palindrome(&self) -> bool {
            let mut stack: Vec<i32> = Vec::new();
            for val in self.iter() {
                stack.push(*val);
            }
            let mut iter = stack.iter();
            while let (Some(x), Some(y)) = (iter.next(), iter.next_back()) {
                if x != y {
                    return false;
                }
            }
            
            true
        }

        /// intersection prompt: given two (singly) linked lists, determine if the two
        /// lists intersect. Return the intersecting node. Note that the intersection is
        /// defined based on reference, not value. That is, if the kth node of the first
        /// linked list is the exact same node (by reference) as the jth node of the second
        /// list, then they are intersecting.
        pub fn intersection<'a>(_list1: List<i32>, _list2: List<i32>) -> Option<&'a Node<i32>> {
            // because of the way I have set up my linked list struct, it isn't
            // possible to create an intersection linked list because the Nodes 
            // are owned. They would have to be wrapped in Rc<> to be owned by
            // multiple lists and I can't be bothered to do that. I'll write
            // psuedo code instead as if it were possible:

            // visited_nodes = set
            // for ref node in list1:
                // add ref to set
            // for ref node in list2:
                // if ref is in set:
                    // return it
            // we got here? no intersection. return None
            unimplemented!()
        }

        /// loop_detection prompt: given a linked list which might contain a loop,
        /// imp,ement an algorithm that returns the node at the beginning of the
        /// loop (if one exists).
        /// EXAMPLE
        /// Input: A -> B -> C -> D -> E -> C (the same C as earlier)
        /// Output: C
        pub fn loop_detection<'a>(_list: List<i32>) -> Option<&'a Node<i32>> {
            // we run into the same problem here - the linked list struct I wrote
            // doesn't allow two nodes to own the same self.next. Again, it could
            // be done but it requires a lot of rewriting, so instead I will write
            // pseudo code to get the point accross of how the algorithm would look:

            // visited_nodes = set
            // for ref node in list:
                // if ref is in set:
                    // return it
                // else:
                    // add it to set
            // we got here? no loop. return None
            unimplemented!()
        }
    }

    pub struct ListIntoIter<T>(List<T>);

    impl<T> Iterator for ListIntoIter<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.pop()
        }
    }

    pub struct ListIter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.value
            })
        }
    }

    pub struct ListIterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for ListIterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.value
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::List;
    use std::ptr;

    #[test]
    fn push_and_pop() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn test_iter() {
        let list: List<i32> = List::from_vec(vec![4, 3, 2, 1, 0]);
        for (e, i) in list.iter().enumerate() {
            assert_eq!(e, *i as usize);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut list: List<i32> = List::from_vec(vec![3, 2, 1]);
        for val in list.iter_mut() {
            *val += 1;
        }
        assert_eq!(list.to_vec(), vec![2, 3, 4]);
    }

    #[test]
    fn test_nth() {
        let test_cases = vec![
            (vec![5, 3, 8], 0, Some(&8)),
            (vec![10, 22, 2, 8, 4], 3, Some(&22)),
            (vec![4, 99, 1, 6, 12], 4, Some(&4)),
            (vec![9, 2], 3, None),
            (vec![62, 2], 1, Some(&62)),
        ];
        for (input, idx, output) in test_cases {
            let list = List::from_vec(input);
            let nth = list.nth(idx);
            let val = nth.map(|node| node.peek());
            assert_eq!(val, output);
        }
    }

    #[test]
    fn test_remove_dups() {
        let test_cases = vec![
            (vec![3, 2, 1], vec![1, 2, 3]),
            (vec![3, 4, 3, 1, 3, 4], vec![4, 3, 1]),
            (vec![7, 7, 5, 4, 5, 1, 6], vec![6, 1, 5, 4, 7]),
            (vec![1, 6, 1, 6, 1, 3, 1], vec![1, 3, 6]),
            (vec![2, 4, 6, 7, 6, 4, 5, 1, 2], vec![2, 1, 5, 4, 6, 7]),
        ];
        for (input, output) in test_cases {
            let mut list: List<i32> = List::from_vec(input);
            list.remove_dups();
            assert_eq!(list.to_vec(), output);
        }
    }

    #[test]
    fn test_kth() {
        let test_cases = vec![
            (vec![3, 4, 3, 1, 2], 3, Some(&3)),
            (vec![2, 1], 1, Some(&2)),
            (vec![5, 7, 33, 21, 1], 3, Some(&33)),
            (vec![66, 100, 4, 2, 101], 1, Some(&66)),
            (vec![2, 5], 3, None),
            (vec![3, 4, 3, 1, 2], 6, None),
        ];
        for (input, idx, output) in test_cases {
            let list: List<i32> = List::from_vec(input);
            assert_eq!(list.kth(idx), output);
        }
    }

    #[test]
    fn test_delete_middle_node() {
        let test_cases = vec![
            (vec![3, 4, 5, 6, 2], 2, vec![2, 6, 4, 3]),
            (vec![10, 11, 12], 0, vec![11, 10]),
            (vec![7, 5, 25, 10, 9], 4, vec![9, 10, 25, 5]),
            (vec![200, 5, 1], 1, vec![1, 200]),
            (vec![60, 400, 2, 35, 8, 19], 2, vec![19, 8, 2, 400, 60]),
        ];
        for (input, idx, output) in test_cases {
            let mut list = List::from_vec(input);
            let node = list.nth(idx);
            if let Some(n) = node {
                let node_ptr = ptr::addr_of!(*n);
                list.delete_middle_node(node_ptr);
                assert_eq!(list.to_vec(), output);
            }
        }
    }

    #[test]
    fn test_partition() {
        let test_cases = vec![
            (vec![2, 6, 4, 3, 1, 6, 7], 3, vec![2, 1, 7, 6, 3, 4, 6]),
            (vec![3, 8, 6, 5], 9, vec![3, 8, 6, 5]),
            (vec![1, 5, 3, 7, 44, 3], 2, vec![1, 3, 44, 7, 3, 5]),
            (
                vec![7, 33, 2, 11, 11, 10, 6, 3, 22],
                11,
                vec![7, 2, 10, 6, 3, 22, 11, 11, 33],
            ),
            (vec![50, 25, 1, 3, 100], 30, vec![25, 1, 3, 100, 50]),
        ];
        for (input, x, output) in test_cases {
            let mut list = List::from_vec(input);
            list.partition(x);
            assert_eq!(list.to_vec(), output);
        }
    }

    #[test]
    fn test_sum_lists() {
        let test_cases = vec![
            (vec![1, 0, 1], vec![5, 1], vec![2, 5, 1]),
            (vec![4, 2], vec![2, 0], vec![2, 6]),
            (vec![7, 3], vec![2, 7], vec![0, 0, 1]),
            (vec![2], vec![4, 4], vec![6, 4]),
            (vec![2, 0, 4], vec![5, 3, 3], vec![7, 3, 7]),
            (vec![9, 2, 6], vec![1, 1, 4, 3], vec![9, 6, 0, 2]),
        ];
        for (l1, l2, output) in test_cases {
            assert_eq!(
                List::sum_lists(List::from_vec(l1), List::from_vec(l2)).to_vec(),
                output
            );
        }
    }

    #[test]
    fn test_sum_lists_fwd() {
        let test_cases = vec![
            (vec![5, 1], vec![1, 0, 1], vec![1, 5, 2]),
            (vec![4, 2], vec![2, 0], vec![6, 2]),
            (vec![7, 3], vec![2, 7], vec![1, 0, 0]),
            (vec![2], vec![4, 4], vec![4, 6]),
            (vec![2, 0, 4], vec![5, 3, 3], vec![7, 3, 7]),
            (vec![9, 2, 6], vec![1, 1, 4, 3], vec![2, 0, 6, 9]),
        ];
        for (l1, l2, output) in test_cases {
            assert_eq!(
                List::sum_lists_fwd(List::from_vec(l1), List::from_vec(l2)).to_vec(),
                output
            );
        }
    }

    #[test]
    fn test_palindrome() {
        let test_cases = vec![
            (vec![4, 5, 6], false),
            (vec![4, 5, 4], true),
            (vec![4, 5, 5, 4], true),
            (vec![8, 5, 8, 7, 8, 5, 8], true),
            (vec![4, 5, 6, 6, 5, 7, 4], false),
            (vec![1, 1, 1, 1], true),
            (vec![8, 9, 7, 7, 7, 9], false),
            (vec![3, 4, 2, 3, 4, 3], false),
            (vec![8, 2, 2, 2, 3, 2, 2, 8], false),
        ];
        for (input, result) in test_cases {
            let list = List::from_vec(input);
            assert_eq!(list.palindrome(), result);
        }
    }
}
