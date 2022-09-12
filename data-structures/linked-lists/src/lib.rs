/// We could write `use std::collections::LinkedList`...
/// Or we could write it ourselves!
pub struct LinkedList<'a, T> {
    head: Node<'a, T>,
}

impl<'a, T: std::clone::Clone> LinkedList<'a, T> {
    fn new(values: Vec<T>) -> Self {
        LinkedList {
            head: Node {
                next: None,
                value: values.last().cloned().unwrap(),
            },
        }
    }
}

/// A single node in the linked list
pub struct Node<'a, T> {
    next: Option<&'a Node<'a, T>>,
    value: T,
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
    // fn test_remove_dups() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
