use z3::{ast::*, *};

pub fn run() {
    let mut cfg = Config::new();
    cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);

    let x = BV::new_const(&ctx, "x", 4);

    let solver = Solver::new(&ctx);
    let two = Int::from_i64(&ctx, 2);
    let bool_vec = &x
        .children()
        .iter()
        .zip(0..4)
        .map(|(x, i)| x.as_int().unwrap()._eq(&two.power(&Int::from_i64(&ctx, i))))
        .collect::<Vec<_>>();

    let bool_vec = bool_vec.iter().map(|y| y).collect::<Vec<_>>();

    let or = Bool::or(&ctx, &bool_vec.as_slice());

    solver.assert(&is_power_of_two(&ctx, &x)._eq(&or));
    println!("{:?}", solver.check());
    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    println!("{}", model.to_string());
    // println!("{:?}", solver.get_proof().unwrap());
}

pub fn is_power_of_two<'ctx>(ctx: &'ctx Context, val: &BV<'ctx>) -> Bool<'ctx> {
    let zero: BV = BV::from_i64(&ctx, 0, 4);
    Bool::and(
        &ctx,
        &[
            &val._eq(&zero).not(),
            &zero._eq(&val.bvand(&val.bvsub(&BV::from_i64(&ctx, 1, 4)))),
        ],
    )
}
