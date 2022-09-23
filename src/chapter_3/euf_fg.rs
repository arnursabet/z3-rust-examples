use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let s_sort = DatatypeBuilder::new(&ctx, "S")
        .variant(
            "Some",
            vec![(
                "value",
                DatatypeAccessor::Sort(Sort::uninterpreted(&ctx, "S".into())),
            )],
        )
        .finish();

    let a = Datatype::new_const(&ctx, "a", &s_sort.sort);
    let b = Datatype::new_const(&ctx, "b", &s_sort.sort);
    let c = Datatype::new_const(&ctx, "c", &s_sort.sort);
    let d = Datatype::new_const(&ctx, "d", &s_sort.sort);
    let e = Datatype::new_const(&ctx, "e", &s_sort.sort);
    let s = Datatype::new_const(&ctx, "s", &s_sort.sort);
    let t = Datatype::new_const(&ctx, "t", &s_sort.sort);

    let f = FuncDecl::new(&ctx, "f", &[&s_sort.sort, &s_sort.sort], &s_sort.sort);
    let g = FuncDecl::new(&ctx, "g", &[&s_sort.sort], &s_sort.sort);

    let solver = Solver::new(&ctx);

    solver.assert(&a._eq(&b));
    solver.assert(&b._eq(&c));
    solver.assert(&d._eq(&e));
    solver.assert(&b._eq(&s));
    solver.assert(&d._eq(&t));

    let g_d = g.apply(&[&d]);
    let f_a_g_d = f.apply(&[&a, &g_d]);
    let g_e = g.apply(&[&e]);
    let f_g_e_b = f.apply(&[&g_e, &b]);

    solver.assert(&f_a_g_d._eq(&f_g_e_b).not());

    assert_eq!(solver.check(), SatResult::Sat);

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
