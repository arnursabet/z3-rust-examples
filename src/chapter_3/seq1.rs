use std::ops::Add;

use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let s = String::new_const(&ctx, "s");
    let t = String::new_const(&ctx, "t");
    let u = String::new_const(&ctx, "u");

    let and = Bool::and(
        &ctx,
        &[
            &s.prefix(&t),
            &u.suffix(&t),
            &Bool::from_bool(
                &ctx,
                s.as_string().unwrap().len().eq(&s
                    .as_string()
                    .unwrap()
                    .len()
                    .add(&u.as_string().unwrap().len())),
            ),
        ],
    );

    let implies = Bool::implies(&and, &t._eq(&String::concat(&ctx, &[&s, &u])));

    let solver = Solver::new(&ctx);

    solver.assert(&implies);

    println!("{:?}", solver.check());

    let model = solver.get_model().unwrap();

    println!("{}", model.to_string());
}
