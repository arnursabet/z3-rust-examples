use z3::{ast::*, *};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    // let s_sort = DatatypeBuilder::new(&ctx, "S")
    //     .variant(
    //         "const",
    //         vec![("const", DatatypeAccessor::Sort(Sort::int(&ctx)))],
    //     )
    //     .finish();

    let s_sort = Sort::uninterpreted(&ctx, "S".into());

    let f = FuncDecl::new(&ctx, "f", &[&s_sort], &s_sort);

    let x = Datatype::new_const(&ctx, Symbol::String("x".to_string()), &s_sort);

    //println!("{}", "hello");
    let solver = Solver::new(&ctx);

    solver.push();
    solver.assert(&f.apply(&[&f.apply(&[&x])]).as_datatype().unwrap()._eq(&x));
    solver.assert(
        &f.apply(&[&f.apply(&[&f.apply(&[&x])])])
            .as_datatype()
            .unwrap()
            ._eq(&x),
    );
    println!("{:?}", solver.check());

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
    println!("{}", solver.to_string());

    solver.pop(1);
    solver.push();
    solver.assert(&f.apply(&[&f.apply(&[&x])]).as_datatype().unwrap()._eq(&x));
    solver.assert(
        &f.apply(&[&f.apply(&[&f.apply(&[&x])])])
            .as_datatype()
            .unwrap()
            ._eq(&x),
    );
    solver.assert(&f.apply(&[&x]).as_datatype().unwrap()._eq(&x).not());

    println!("{:?}", solver.check());

    let model = solver.get_model();
    assert!(model.is_some());
    let model = model.unwrap();
    let model_str = format!("{:?}", model);
    eprintln!("{}", model_str);
}
