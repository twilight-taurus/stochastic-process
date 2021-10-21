use rand::Rng;

//use crate::base::*;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Node
{
    pub current: Point,
}

impl Node {
    pub fn new(current: Point) -> Self {
        Self {
            current: current,
        }
    }
}

#[derive(Debug)]
pub struct GeometricBrownianMotion
{
    calculated_values: std::vec::Vec<Point>,

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
            calculated_values: std::vec::Vec::with_capacity(loops as usize),
            // model parameters 
            initial: initial,

            drift: drift,
            volatility: volatility,

            step: step,
            distance: loops as f32 * step,
        }
    }

    #[inline]
    fn push_back(&mut self, point: Point) {

//        self.calculated_values.push(point);
//        self.len += 1;
    }

    // generate motion from given attributes
    pub fn generate(&mut self) {
        while self.distance > 0.0 {
            self.generate_single();
        }
    }
    // generate single value
    pub fn generate_single(&mut self) {
        let mut point = Point { x: 0.0, y: 0.0};

        point.y = self.initial.y * self.step * (self.drift as f32) +
                    self.volatility * ( rand::thread_rng().gen_range(0..(self.step.sqrt() as u32)) as f32 );
        point.x = self.step;
        self.calculated_values.push( point );

        loop {
            point.y = point.y * self.step * (self.drift as f32) +
            self.volatility * ( rand::thread_rng().gen_range(0..(self.step.sqrt() as u32)) as f32 );
            point.x += point.x;

            self.calculated_values.push( point );

            self.distance -= self.step;

            if self.distance <= 0.0 {
                break;
            }
        }
    }
    // generate with different params
    pub fn generate_more(&mut self, n: u32, initial: u32, drift: u32, volatility: u32, delta: f32, total_time: f32) {

    }
    pub fn reset(&mut self) {
        self.calculated_values.clear();
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