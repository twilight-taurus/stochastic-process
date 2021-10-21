use rand::Rng;

use crate::base::*;

#[derive(Debug, Clone)]
pub struct GeometricBrownianMotion
{
    head: Option<*mut Node>,
    tail: Option<*mut Node>,

    len: usize,

    initial: Point, // starting point

    drift: f32, // stochastic drift: max. change from the average value in the stochastic process.
    volatility: f32, // stochastic volatility
                    // assumption: volatility in prices/etc is not constant over time.
                    // purpose: value allows volatility in the underlying object to fluctuate over time.

    step: f32, // distance per loop/step.
    distance: f32, // total distance: multiple of step
}

impl ProcessIntoIterator for GeometricBrownianMotion {
    type Item = *mut Node;
    type ProcessIntoIter = ProcessIterMut;

    fn into_iter(&mut self) -> Self::ProcessIntoIter {
        ProcessIterMut {
            head: self.head,
            tail: self.tail,
            len: (self.distance / self.step) as usize,
        }
    }
}

impl Process for GeometricBrownianMotion {
    #[inline]
    fn push_back(&mut self, mut boxed: Box<Node>) -> usize {

        boxed.next = None;
        boxed.prev = self.tail; // assign current tail node of container to prev pointer of new node. 
                                // -> (the container's last element)
        unsafe {
            let node: *mut Node = Box::leak(boxed);

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

    // generate motion from given attributes
    fn generate(&mut self) {  
        while self.distance > 0.0 {
            self.generate_single();
        }
    }
    // generate single value
    fn generate_single(&mut self) {
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
        }

        self.distance -= self.step;
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

struct StandardBrownianMotion {
    drift: Option<f32>, // with or without drift parameter determined at initialization.
}