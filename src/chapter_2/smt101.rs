use std::ops::Add;

use z3::{
    ast::{Array, Ast, Bool, Int},
    Config, Context, FuncDecl, SatResult, Solver, Sort,
};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let z_sort = Sort::int(&ctx);

    let f = FuncDecl::new(&ctx, "f", &[&z_sort], &z_sort);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");

    let one = Int::from_i64(&ctx, 1);
    let two = Int::from_i64(&ctx, 2);
    let three = Int::from_i64(&ctx, 3);

    let arr = Array::new_const(&ctx, "A", &z_sort, &z_sort);

    let x_plus_two_eq_y = Int::add(&ctx, &[&x, &two])._eq(&y);
    let y_minus_x_plus_one = Int::sub(&ctx, &[&y, &x]).add(&one);
    let y_minus_two = Int::sub(&ctx, &[&y, &two]);

    let f_a = f.apply(&[&arr.store(&x, &three).select(&y_minus_two)]);
    let f_b = f.apply(&[&y_minus_x_plus_one]);

    let fml = Bool::implies(&x_plus_two_eq_y, &f_a._eq(&f_b));

    let s = Solver::new(&ctx);

    s.assert(&fml);
    let solution = s.check();
    assert_eq!(solution, SatResult::Sat);

    let model = s.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
