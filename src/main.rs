use Foo::brownian::*;
use Foo::base::*;

fn main() {
    let mut gbm = 
        GeometricBrownianMotion::new(Point {x: 0.0, y: 0.0}, 21.0, 8.0, 6.0, 10);

    gbm.generate_single();
    gbm.generate_single();
    gbm.generate_single();

    return;
}