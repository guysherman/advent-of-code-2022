// Somewhat borrowed from / inspired by https://rust-unofficial.github.io/too-many-lists

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node::<T> {
            elem,
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(node);
            return
        }

        let mut current = self.head.as_mut();
        while let Some(mut boxed_node) = current {
            if boxed_node.next.is_none() {
                boxed_node.next = Some(node);
                break;
            }
            current = boxed_node.next.as_mut();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut boxed_node) = current {
            current = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}



//impl<T> Iter for List<T> {
    //type Item = T;

    
//}
//
//
#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn given_1_2_3_pushed_pops_1_2_3() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn given_1_2_3_pushed_iter_produces_1_2_3() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut it = list.iter();
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn given_1_2_3_pushed_iter_mut_produces_1_2_3_muts() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut it = list.iter_mut();
        assert_eq!(it.next(), Some(&mut 1));
        assert_eq!(it.next(), Some(&mut 2));
        assert_eq!(it.next(), Some(&mut 3));
        assert_eq!(it.next(), None);
    }
}
