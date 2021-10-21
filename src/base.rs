// basic traits and types implemented by the structs.

pub trait Process {
    fn generate(&mut self); // calculate multiple steps

    fn generate_single(&mut self); // calculate a single step.

    fn reset(&mut self); // remove all elements

    fn push_back(&mut self, boxed: Box<Node>) -> usize; // add an element to back

    fn pop_back(&mut self) -> Option<Box<Node>>; // remove an element from back
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Node
{
    pub current: Point,
    pub next: Option<*mut Node>,
    pub prev: Option<*mut Node>,
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
pub struct ProcessIter {
    pub head: Option<*mut Node>,
    pub tail: Option<*mut Node>,
    pub len: usize,
}

pub trait ProcessIntoIterator {
    type Item;
    type ProcessIntoIter;

    fn into_iter(&mut self) -> Self::ProcessIntoIter; 
}

// iterator one-ended
impl<'a> Iterator for ProcessIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            None => {
                None
            },
            Some(head) => {
                unsafe {
                    self.head = (*head).next;
                    // return copy of current element, not next element.
                    Some((*head).current)
                }
            }
        }
    }
}