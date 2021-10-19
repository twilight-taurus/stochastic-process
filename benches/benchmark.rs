use criterion::{black_box, criterion_group, criterion_main, Criterion};



use process_lib::brownian::*;
//use process_lib::brownian_vec::*;
use process_lib::base::Point;

fn criterion_benchmark(c: &mut Criterion) {
    let mut gbm = GeometricBrownianMotion::new(Point {x: 0.0, y: 0.0}, 6.0, 10.0, 1.0, 1000);

    c.bench_function(
        "generate 1000",
        |b| b.iter( || {
            gbm.generate(); 
        })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);