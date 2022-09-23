use std::ops::Add;

use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");

    let solver = Solver::new(&ctx);

    let zero = Real::from_real(&ctx, 0, 1);
    let two = Real::from_real(&ctx, 2, 1);
    let four = Real::from_real(&ctx, 4, 1);
    let six = Real::from_real(&ctx, 6, 1);
    let two_y = Real::mul(&ctx, &[&two, &y]);

    solver.assert(&x.ge(&zero));
    solver.assert(&Bool::or(
        &ctx,
        &[&x.clone().add(&y).le(&two), &x.clone().add(&two_y).ge(&six)],
    ));
    solver.assert(&Bool::or(
        &ctx,
        &[&x.clone().add(&y).ge(&two), &x.add(&two_y).ge(&four)],
    ));

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
