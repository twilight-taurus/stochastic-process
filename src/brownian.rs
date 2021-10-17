//#![feature(box_into_inner)]

use std::{marker::PhantomData, ops::Mul};

use num_integer::Roots;
use rand::Rng;
use num_traits::{PrimInt, Float, NumOps, FromPrimitive, ToPrimitive, NumAssign};
use num_traits::cast::NumCast;
use num_traits::bounds::Bounded;

extern crate num_integer;
extern crate num_traits;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Node
{
    current: Point,
    next: Option<*mut Node>,
    prev: Option<*mut Node>
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

#[derive(Debug)]
pub struct GeometricBrownianMotion
{
    pub head: Option<*mut Node>,
    pub tail: Option<*mut Node>,

    len: usize,

    initial: Point, // starting point

    drift: f32, // stochastic drift: max. change from the average value in the stochastic process.
    volatility: f32, // stochastic volatility
                    // assumption: volatility in prices/etc is not constant over time.
                    // purpose: value allows volatility in the underlying object to fluctuate over time.

    step: f32, // distance per loop/step.
    distance: f32, // total distance: multiple of step
}


impl GeometricBrownianMotion {
    pub fn new(initial: Point, drift: f32, volatility: f32, step: f32, loops: u32) -> Self {
        Self {
            head: None,
            tail: None,

            len: 0,

            // model parameters
            initial: initial,
            drift: drift,
            volatility: volatility,
            step: step,
            distance: loops as f32 * step,
        }
    }

    #[inline]
    fn push_back(&mut self, mut boxed: Box<Node>) {
    
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
    }
    #[inline]
    fn pop_back(&mut self) -> Option<Box<Node>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node);
            self.tail = node.prev;
        });

        let result: Option<Box<Node>> = match self.tail {
            None => {
                // no nodes available.
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

    // generate motion from given attributes
    pub fn generate(&mut self) {
        while self.distance > 0.0 {
            self.generate_single();
            self.distance -= self.step;
            
        }
    }
    // generate single value
    pub fn generate_single(&mut self) {
        if let Some(tail) = self.tail {
            unsafe {
                let cur = &(*tail).current;

                let res= cur.y * self.step * (self.drift as f32) +
                    self.volatility * ( rand::thread_rng().gen_range(0..(self.step.sqrt() as u32)) as f32 );
                
                let boxed = Box::new( Node::new(
                    Point {
                                x: cur.x + self.step, y: res
                            })
                );        
                self.push_back(boxed);
                self.len += 1;
            }
        } else {
            let res = self.initial.y * self.step * self.drift +
                self.volatility * ( rand::thread_rng().gen_range(0..(self.step.sqrt() as u32)) as f32 );

            let boxed = Box::new( Node::new(
                Point {
                            x: self.initial.x + self.step, y: res
                        })
            );
            self.push_back(boxed);
            self.len += 1;
        }
    }
    // generate with different params
    pub fn generate_more(&mut self, n: u32, initial: u32, drift: u32, volatility: u32, delta: f32, total_time: f32) {

    }
    pub fn reset(&mut self) {

    }
    pub fn set_drift(&mut self, drift: f32) {
        self.drift = drift;
    }
    pub fn set_volatility(&mut self, volatility: f32) {
        self.volatility = volatility
    }
    pub fn set_step(&mut self, step: f32) {
        self.step = step;
    }
}