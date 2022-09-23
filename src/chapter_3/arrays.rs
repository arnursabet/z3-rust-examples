use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let arr = Array::new_const(&ctx, "A", &Sort::int(&ctx), &Sort::int(&ctx));
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let solver = Solver::new(&ctx);

    solver.assert(&arr.select(&x).as_int().unwrap()._eq(&x));
    solver.assert(&arr.store(&x, &y)._eq(&arr));

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
