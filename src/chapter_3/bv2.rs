use z3::{ast::*, *};

pub fn run() {
    let mut cfg = Config::new();
    cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);

    let v = BV::new_const(&ctx, "v", 32);

    let mask = v.bvashr(&BV::from_i64(&ctx, 31, 32));
    let solver = Solver::new(&ctx);

    solver.assert(
        &Bool::ite(
            &v.bvsgt(&BV::from_i64(&ctx, 0, 32)),
            &v,
            &v.bvmul(&BV::from_i64(&ctx, -1, 32)),
        )
        ._eq(&v.bvadd(&mask).bvxor(&mask)),
    );

    println!("{:?}", solver.check());
    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    println!("{}", model.to_string());
    // println!("{:?}", solver.get_proof().unwrap());
}
