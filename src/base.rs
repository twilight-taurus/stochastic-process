// basic traits and types implemented by the structs.
use std::marker::PhantomData;


pub trait Process {
    fn generate(&mut self); // calculate multiple steps

    fn generate_single(&mut self); // calculate a single step.

//    fn generate_more(self);

//    fn reset(self); // remove all elements

    fn push_back(&mut self, boxed: Box<Node>); // add an element to back

    fn pop_back(&mut self) -> Option<Box<Node>>; // remove an element from back
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
pub struct ProcessIter<Node> {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
    // marker to prevent compiler form complaining about lifetime specifier 'a
    marker: PhantomData<*mut Node>
}

// iterator one-ended
impl Iterator for ProcessIter<Node> {
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