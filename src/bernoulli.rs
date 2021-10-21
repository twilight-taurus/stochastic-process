use rand::Rng;

use crate::base::*;

#[derive(Debug)]

pub struct Bernoulli {
    head: Option<*mut Node>,
    tail: Option<*mut Node>,

    len: usize,

    initial: Point,

    step: Point, // x- and y-difference between outcome 0 and 1
    distance: f32, // total distance: multiple of step.x
}

impl Process for Bernoulli {
    #[inline]
    fn push_back(&mut self, mut boxed: Box<Node>) -> usize {

        boxed.next = None;
        boxed.prev = self.tail; // assign current tail node of container to prev pointer of new node. 
                                // -> (the container's last element)
        unsafe {
            let node = Box::leak(boxed); // leak the box.

            match self.tail {
                // no elements in container. create head node.
                None => self.head = Some(node),
                // elements exist. assign next pointer of tail to new node.
                Some(tail) => (*tail).next = Some(node),
            }

            // independently of whether tail exists, assign current node to tail.
            self.tail = Some(node);
        }
        self.len += 1;
        self.len
    }
    #[inline]
    fn pop_back(&mut self) -> Option<Box<Node>> {
        let result = match self.tail {
            None => {
                // hint: not necessary to Box::from_raw(self.head), since the memory is automatically boxed when
                // the first self.tail is popped (below)
                self.head = None;
                None
            }
            Some(tail) => {
                unsafe {
                    let mut boxed = Box::from_raw(tail);
                    boxed.next = None;
                    if let Some(prev) = boxed.prev {
                        self.tail = Some(prev);
                    }
                    Some(boxed)
                }
            }
        };

        result
    }
    fn reset(&mut self) {
        // if stopped, either end reached (reset complete), or linked list is broken.
        unsafe {
            // tail: currently the last node.
            while let Some(tail) = self.tail {

                // box the value. making it memory safe.
                let boxed = Box::from_raw(tail);

                boxed.prev.map( |node| {
                    (*node).next = None;
                });
                self.tail = boxed.prev;
            }
            self.head = None;
        }
    }
    fn generate(&mut self) {
        while self.distance > 0.0 {
            self.generate_single();
        }
    }
    fn generate_single(&mut self) {
        let mut point: Point = Point {x: 0.0, y: 0.0 };

        if let Some(tail) = self.tail {
            unsafe {
                let cur = &(*tail).current;
                point.x = cur.x + self.step.x;
            }
            if rand::thread_rng().gen_ratio(1, 2) {
                // upper 50% : move up 
                point.y = self.initial.y + self.step.y;
            } else {
                // lower 50% : stay
                point.y = self.initial.y;
            }
        } else {
            point.x = self.initial.x;

            if rand::thread_rng().gen_ratio(1, 2) {
                // upper 50% : move up 
                point.y = self.initial.y + self.step.y;
            } else {
                // lower 50% : stay
                point.y = self.initial.y - self.step.y;
            }      
        }

        let boxed = Box::new( Node::new(point) );
        self.push_back(boxed);

        self.distance -= self.step.x;
    }
}

impl Bernoulli {
    pub fn new(initial: Point, step: Point, steps: u32) -> Self {
        Self {
            head: None,
            tail: None,

            len: 0,

            // model parameters
            initial: initial,
            step: step,
            distance: steps as f32 * step.x,
        }
    }
}