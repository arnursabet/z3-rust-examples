use z3::{
    ast::{Ast, Int},
    Config, Context,
};

pub fn run() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let n = Int::add(&ctx, &[&x, &y]).ge(&Int::from_i64(&ctx, 3));

    println!("{}", n.num_children());
    println!("{:?}", n.children());
    println!("{:?}", n.nth_child(0).unwrap());
    println!("{:?}", n.nth_child(1).unwrap());
    println!("{:?}", n.decl());
    println!("{:?}", n.decl().name());
}
