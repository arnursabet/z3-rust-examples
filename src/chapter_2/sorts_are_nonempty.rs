use z3::{
    ast::{forall_const, Ast, Datatype},
    Config, Context, DatatypeAccessor, DatatypeBuilder, SatResult, Solver, Sort,
};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let s_sort = DatatypeBuilder::new(&ctx, "S")
        .variant(
            "const",
            vec![("const", DatatypeAccessor::Sort(Sort::string(&ctx)))],
        )
        .finish();

    let s = Datatype::new_const(&ctx, "s", &s_sort.sort);

    let solver = Solver::new(&ctx);

    let forall = forall_const(&ctx, &[&s], &[], &s._eq(&s).not());

    solver.assert(&forall);
    assert_eq!(solver.check(), SatResult::Sat);
}
