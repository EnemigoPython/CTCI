pub mod linked_list {
    use std::fmt::Debug;
    use std::{collections::HashSet, hash::Hash};

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
    }

    // challenge impl
    impl<T> List<T>
    where
        T: Hash + Eq + Clone + Debug,
    {
        /// remove_dups prompt: Write code to remove duplicates from an unsorted linked list.
        /// FOLLOW UP:
        /// How would you solve this problem if a temporary buffer is not allowed?
        pub fn remove_dups(&mut self) {
            let mut unique: HashSet<T> = HashSet::new();
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
        pub fn kth(&self, k: usize) -> Option<&T> {
            let to_vec: Vec<&T> = self.iter().collect();
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
        pub fn delete_middle_node(&mut self, del_node: &Node<T>) {

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
            (vec![3, 4, 5, 6, 2], 2, vec![3, 4, 6, 2]),
        ];
        for (input, idx, output) in test_cases {
            let start_range = 0..idx+1;
            let start_slice: Vec<i32> = (&input[start_range]).to_owned();
            let mut list = List::from_vec(start_slice);
            let node_ref = list.head.as_deref();
            let end_range = idx+1..input.len();
            let end_slice: Vec<i32> = (&input[end_range]).to_owned();
            println!("{end_slice:?}");
            for val in end_slice {
                list.push(val);
            }
            assert_eq!(true, false);
        }
    }
}
