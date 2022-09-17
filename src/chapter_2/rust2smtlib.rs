use z3::{ast::Int, Config, Context, Solver};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let s = Solver::new(&ctx);

    let x_mod_four = Int::modulo(&x, &Int::from_i64(&ctx, 4));
    let three_mul_y_div_two = Int::mul(
        &ctx,
        &[
            &Int::from_i64(&ctx, 3),
            &Int::div(&y, &Int::from_i64(&ctx, 2)),
        ],
    );

    let x_minus_y = Int::sub(&ctx, &[&x, &y]);

    s.assert(&Int::add(&ctx, &[&x_mod_four, &three_mul_y_div_two]).gt(&x_minus_y));

    println!("{}", s.to_string());
}
