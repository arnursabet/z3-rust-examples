use std::ops::Add;

use z3::{
    ast::{Bool, Int},
    Config, Context, FuncDecl, SatResult, Solver, Sort,
};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let z_sort = Sort::int(&ctx);
    let bool_sort = Sort::bool(&ctx);

    let f = FuncDecl::new(&ctx, "f", &[&bool_sort], &z_sort);
    let g = FuncDecl::new(&ctx, "g", &[&z_sort], &bool_sort);

    let a = Bool::new_const(&ctx, "a");

    let one = Int::from_i64(&ctx, 1);

    let solver = Solver::new(&ctx);

    let one_plus_f_a = one.add(&f.apply(&[&a]).as_int().unwrap());
    let expression = g.apply(&[&one_plus_f_a]).as_bool().unwrap();

    solver.assert(&expression);

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
