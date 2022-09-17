use std::convert::TryInto;

use z3::{ast::*, *};

pub fn run() {
    let mut cfg = Config::new();
    let _ = cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);

    let int_sort = Sort::int(&ctx);
    let bool_sort = Sort::bool(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let q = Array::new_const(&ctx, "Q", &int_sort, &bool_sort);

    let solver = Solver::new(&ctx);

    let implies = Bool::implies(
        &q.select(&x).as_bool().unwrap(),
        &q.select(&y).as_bool().unwrap(),
    );

    let forall = forall_const(&ctx, &[&q], &[], &implies).try_into().unwrap();

    let fml = Bool::implies(&forall, &x._eq(&y));

    solver.assert(&fml);

    println!("{:?}", solver.check());

    // Figure out why the code below panics
    //let proof = solver.get_proof().unwrap();

    //println!("{:?}", proof);
}
