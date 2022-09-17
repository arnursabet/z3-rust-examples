use z3::{ast::Bool, Config, Context, SatResult, Solver};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let tie = Bool::new_const(&ctx, "tie");
    let shirt = Bool::new_const(&ctx, "shirt");
    let s = Solver::new(&ctx);

    s.assert(&Bool::or(&ctx, &[&tie, &shirt]));
    s.assert(&Bool::or(&ctx, &[&Bool::not(&tie), &shirt]));
    s.assert(&Bool::or(&ctx, &[&Bool::not(&tie), &Bool::not(&shirt)]));

    let solution = s.check();
    assert_eq!(solution, SatResult::Sat);

    let model = s.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
