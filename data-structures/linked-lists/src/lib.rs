pub mod linked_list {
    type Link<T> = Option<Box<Node<T>>>;
    
    struct Node<T> {
        value: T,
        next: Link<T>,
    }
    
    pub struct List<T> {
        head: Link<T>,
    }
    
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
            let mut v: Vec<T> = Vec::new();
            for i in self.into_iter() {
                v.push(i);
            }
            v
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


/// remove_dups prompt: Write code to remove duplicates from an unsorted linked list.
/// FOLLOW UP:
/// How would you solve this problem if a temporary buffer is not allowed?

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
}
