use z3::{ast::*, *};
pub fn run() {
    let mut cfg = Config::new();
    cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let tree_builder = DatatypeBuilder::new(&ctx, "Tree")
        .variant("Empty", vec![])
        .variant(
            "Node",
            vec![
                ("left", DatatypeAccessor::Datatype("Tree".into())),
                ("data", DatatypeAccessor::Sort(Sort::int(&ctx))),
                ("right", DatatypeAccessor::Datatype("Tree".into())),
            ],
        );
    let tree_sort = &z3::datatype_builder::create_datatypes(vec![tree_builder])[0];
    let tree = tree_sort.variants[0].constructor.apply(&[]);
    let t = z3::ast::Datatype::new_const(&ctx, "t", &tree_sort.sort);

    solver.assert(&t._eq(&tree.as_datatype().unwrap()).not());

    println!("{:?}", solver.check());

    let model = solver.get_model().unwrap();

    println!("{}", model.to_string());

    let tree_node_zero =
        tree_sort.variants[1]
            .constructor
            .apply(&[&t, &Int::from_i64(&ctx, 0), &t]);

    solver.assert(&t._eq(&tree_node_zero.as_datatype().unwrap()).not());

    println!("{:?}", solver.check());

    println!("{:?}", solver.get_proof());
}
