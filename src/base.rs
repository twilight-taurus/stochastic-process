// basic traits and types implemented by the structs.
use std::{marker::PhantomData, ops::Mul};


trait Process {
    fn new() -> Self;

    fn generate(self);

    fn generate_single(self);

    fn generate_more(self);

    fn reset(self);

    fn push_back(self);

    fn pop_back(self);
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Node
{
    pub current: Point,
    pub next: Option<*mut Node>,
    pub prev: Option<*mut Node>
}

impl Node {
    pub fn new(current: Point) -> Self {
        Self {
            current: current,
            next: None,
            prev: None,
        }
    }
}

// struct created by calling iter() method on the model struct.
// -> move to base.rs as a base for all model types.
pub struct Iter<Node> {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
    // marker to prevent compiler form complaining about lifetime specifier 'a
    marker: PhantomData<*mut Node>
}

// iterator one-ended
impl Iterator for Iter<Node> {
    type Item = Point;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            None => {
                None
            },
            Some(head) => {
                unsafe {
                    let node = &*head;
                    self.head = node.next;
                    // return copy of current element, not next element.
                    Some(node.current)
                }
            }
        }
    }
}