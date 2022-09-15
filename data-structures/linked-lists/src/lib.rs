/// A single node in the linked list
pub struct Node <T> {
    next: Box<Option<Node<T>>>,
    value: T,
}

/// We could write `use std::collections::LinkedList`...
/// Or we could write it ourselves!
pub struct LinkedList <T: Copy> {
    head: Node<T>,
}


impl<T: Copy> LinkedList <T> {
    fn new(_value: T) -> Self {
        LinkedList {
            head: Node {
                next: Box::new(None),
                value: _value,
            },
        }
    }

    fn append(&mut self, _value: T) {
        let mut curr_node = &mut *self.head.next;
        loop {
            match curr_node {
                Some(node) => curr_node = &mut *node.next,
                None => break,
            }
        }
        curr_node.as_mut().unwrap().next = Box::new(Some(Node {
            next: Box::new(None),
            value: _value,
        }));
    }

    fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter { curr: &self.head }.into_iter()
    }
    
    // fn from_vec(values: Vec<T>) -> Self {
    //     let mut node_refs: Vec<Option<Node<T>>>;
    //     node_refs.push(None);
    //     for item in values {
    //         let new = Node {
    //             next: node_refs.pop().unwrap().as_ref(),
    //             value: item,
    //         };
    //         node_refs.push(Some(new));
    //     }
                
    //     LinkedList {
    //         head: node_refs.pop().unwrap().unwrap(),
    //     }
    // }
}
                
/// Wrapper object for Linked List iteration with iter() 
pub struct LinkedListIter <'a, T: Copy> {
    curr: &'a Node<T>,
}

impl <'a, T: Copy> Iterator for LinkedListIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &*self.curr.next {
            Some(x) => Some(x.value),
            None => None,
        }
    }
}

/// prompt: Write code to remove duplicates from an unsorted linked list.
/// FOLLOW UP:
/// How would you solve this problem if a temporary buffer is not allowed?
pub fn remove_dups(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn append() {
    //     let list = LinkedList::new(3);
    //     for x in list.iter() {
    //         println!("{x}");
    //     }
    // }
    
    // #[test]
    // fn test_linked_list() {
    //     let linked_list = LinkedList::from_vec(vec![3, 4, 5, 6]);
    //     let curr = linked_list.head;
    //     println!("{}", curr.value);
    //     while let Some(x) = curr.next {
    //         println!("{}", x.value);
    //     }
    // }

    // #[test]
    // fn test_remove_dups() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
