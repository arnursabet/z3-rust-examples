use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Float::new_const(&ctx, "x", 8, 24);
    let ten = Float::from_f32(&ctx, 10.0);

    let sum = ten.add_towards_zero(&x);
    println!("{:?}", sum);
}
