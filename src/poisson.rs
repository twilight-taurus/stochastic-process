// --- theory :
// Average time between events is known, but timing is unknown.
// -> events are randomly placed. only average is known (once every 60 minutes on average for example)
// -> so it could aswell take 2 hours (120 minutes) until something happens.

// events are independent of eachother -> one does not influence the following.
// two events cannot occur at the same time.

// --- here:
// replace time as the x-axis variable with the coordinates of x-axis.

/* 

P(k events in time period) = e^(-events/time * time_period) * ...

*/

use rand::Rng;

use crate::base::*;

#[derive(Debug, Clone)]

pub struct Poisson
{
    head: Option<*mut Node>,
    tail: Option<*mut Node>,

    len: usize,

    initial: Point, // starting point

    current_dist: f32, // current time ( -> as total distance )
    average_dist: f32, // average time it takes for event to occur. ( -> as average distance )
    total_dist: f32, // total time used for generation. ( -> as total distance.)
}