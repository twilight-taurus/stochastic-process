use process_lib::brownian::*;
use process_lib::base::*;



fn main() {
    let mut gbm = 
        GeometricBrownianMotion::new(Point {x: 0.0, y: 0.0}, 21.0, 8.0, 6.0, 10);

    gbm.generate();

    return ();
}