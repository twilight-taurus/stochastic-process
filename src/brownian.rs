use num_integer::Roots;
use rand::Rng;


#[derive(Debug)]
struct GBMNode<T> {
    current: T,
    next: Option<GBMNode<T>,
}

impl Iterator for GBMNode<T> {
    // refer to this type using Self::Item
    type Item = T;

    // next is only required method for iterator.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = next {
            self.current = self.next.current;
        } else  {
            Some(current)
        }
    }
}

#[derive(Debug)]
struct GeometricBrownianMotion<T> {
    n: u32; // max values limit

    initial: T; // starting value

    calculated_values: std::vec::Vec<GBMNode<T>>;
    drift: u32; // stochastic drift: max. change from the average value in the stochastic process.
    volatility: u32; // stochastic volatility
                    // assumption: volatility in prices/etc is not constant over time.
                    // purpose: value allows volatility in the underlying object to fluctuate over time.

    delta: f32; // time step. [0, 1] (ideally)
    total_time: f32; // total time: multiple of delta (ideally)
}

impl Default for GeometricBrownianMotion {
    fn default() -> Self {
        GeometricBrownianMotion {
            simulations: std::vec::Vec::new(),
            n: 1000,
            initial: 0,
            current: 0,
            drift: 70,
            volatility: 50,
            delta: 0.3,
            timer: delta * 50.0,
        }
    }
}

impl GeometricBrownianMotion {
    pub fn new(n: u32, initial: u32, drift: u32, volatility: u32, delta: f32, total_time: f32) -> Self {
        Self {
            simulations: std::vec::Vec::new(),
            n: n,
            initial: initial,
            drift: drift,
            volatility: volatility,
            delta: delta,
            timer: total_time,
        }
    }

    // generate motion from given attributes
    pub fn generate(&mut self) {
        while (timer > 0.0) {
            let dS = self.current*self.drift*self.delta +
                self.current*self.volatility*rand::thread_rng().gen_range( 0..self.delta.sqrt() );

            self.calculated_values.push(self.current + dS as u32);
            self.current = dS as u32;
            self.timer -= self.delta;
        }
    }
    // generate single value
    pub fn generate_single(&mut self) {

    }
    // generate with different params
    pub fn generate_more(&mut self, n: u32, initial: u32, drift: u32, volatility: u32, delta: f32, total_time: f32) {

    }
    pub fn reset(&mut self) {

    }
    pub fn set_drift(&mut self) {

    }
    pub fn set_volatility(&mut self) {

    }
    pub fn set_delta(&mut self) {

    }
    pub fn set_limit(&mut self, limit: u32) {
        if limit > n {
            self.n = limit;
        } else {
            self.reset();
        }
    }
    pub fn set_timer(&mut self, timer: f32) {
        self.timer = timer;
    }
}