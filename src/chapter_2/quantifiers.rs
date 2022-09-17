use std::ops::Add;

use z3::ast::forall_const;
use z3::ast::Ast;
use z3::ast::Bool;
use z3::ast::Int;
use z3::Config;
use z3::Context;
use z3::SatResult;
use z3::Solver;

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let solver = Solver::new(&ctx);

    let forall = forall_const(
        &ctx,
        &[&y],
        &[],
        &Bool::implies(&y.le(&Int::from_i64(&ctx, 0)), &x.lt(&y)),
    );

    solver.push();
    solver.assert(&y._eq(&x.clone().add(&Int::from_i64(&ctx, 1))));
    solver.assert(&forall);
    println!("{:?}", solver.check());

    solver.pop(1);

    let forall = forall_const(
        &ctx,
        &[&y],
        &[],
        &Bool::implies(&y.le(&Int::from_i64(&ctx, 0)), &x.gt(&y)),
    );

    solver.push();
    solver.assert(&y._eq(&x.clone().add(&Int::from_i64(&ctx, 1))));
    solver.assert(&forall);
    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
