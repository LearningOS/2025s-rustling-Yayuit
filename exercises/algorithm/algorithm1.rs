use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: u32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: u32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.start.map(|node_ptr| unsafe {
            let node = Box::from_raw(node_ptr.as_ptr());
            self.start = node.next;
            self.length -= 1;
            if self.start.is_none() {
                self.end = None;
            }
            node.val
        })
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();
        let mut a_val = list_a.pop_front();
        let mut b_val = list_b.pop_front();

        loop {
            match (a_val.take(), b_val.take()) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        merged_list.add(a);
                        a_val = list_a.pop_front();
                        b_val = Some(b);
                    } else {
                        merged_list.add(b);
                        a_val = Some(a);
                        b_val = list_b.pop_front();
                    }
                }
                (Some(a), None) => {
                    merged_list.add(a);
                    while let Some(val) = list_a.pop_front() {
                        merged_list.add(val);
                    }
                    break;
                }
                (None, Some(b)) => {
                    merged_list.add(b);
                    while let Some(val) = list_b.pop_front() {
                        merged_list.add(val);
                    }
                    break;
                }
                (None, None) => break,
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as u32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &expected) in target_vec.iter().enumerate() {
            assert_eq!(expected, *list_c.get(i as u32).unwrap());
        }
    }
}