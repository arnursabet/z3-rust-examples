use std::ffi::{CStr, CString};
use std::os::raw::c_uint;
use z3::AstKind::Quantifier;
use z3::Symbol;
use z3::{
    ast::{Array, Ast, Bool, Dynamic, Int},
    Config, Context, RecFuncDecl, Solver, Sort,
};
use z3_sys::{
    Z3_ast, Z3_ast_vector_push, Z3_get_as_array_func_decl, Z3_mk_and, Z3_mk_app, Z3_mk_array_sort,
    Z3_mk_array_sort_n, Z3_mk_ast_vector, Z3_mk_bool_sort, Z3_mk_bound, Z3_mk_config, Z3_mk_const,
    Z3_mk_const_array, Z3_mk_eq, Z3_mk_ite, Z3_mk_lambda, Z3_mk_lambda_const, Z3_mk_le,
    Z3_mk_pattern, Z3_mk_select, Z3_mk_simple_solver, Z3_mk_store, Z3_mk_store_n, Z3_mk_string,
    Z3_mk_string_sort, Z3_mk_unsigned_int, Z3_model_to_string, Z3_pattern_to_ast, Z3_solver_assert,
    Z3_solver_check, Z3_solver_get_model, Z3_sort_to_ast, Z3_string, Z3_to_app, _Z3_sort,
    Z3_L_TRUE,
};
use z3_sys::{Z3_mk_context, Z3_mk_quantifier};
use z3_sys::{Z3_mk_int_sort, Z3_mk_string_symbol, _Z3_context};

pub fn run() {
    unsafe {
        let cfg = Z3_mk_config();
        let ctx = Z3_mk_context(cfg);

        let int_sort = Z3_mk_int_sort(ctx);

        let str_x = CString::new("x").unwrap();
        let str_y = CString::new("y").unwrap();
        let str_z = CString::new("z").unwrap();

        let sym_x = Z3_mk_string_symbol(ctx, str_x.as_ptr());
        let sym_y = Z3_mk_string_symbol(ctx, str_y.as_ptr());
        let sym_z = Z3_mk_string_symbol(ctx, str_z.as_ptr());

        let const_x = Z3_mk_const(ctx, sym_x, int_sort);
        let const_y = Z3_mk_const(ctx, sym_y, int_sort);
        let const_z = Z3_mk_const(ctx, sym_z, int_sort);

        let m = Z3_mk_array_sort(ctx, int_sort, int_sort);
        let m1 = Z3_mk_array_sort(ctx, int_sort, int_sort);
        let bool_sort = Z3_mk_bool_sort(ctx);
        let memset = |ctx, lo, hi, y, m: *mut _Z3_sort| -> Z3_ast {
            let arr = Z3_mk_array_sort(ctx, int_sort, bool_sort);
            let arr = Z3_sort_to_ast(ctx, arr);
            let zero = Z3_mk_unsigned_int(ctx, 0, int_sort);
            let one = Z3_mk_unsigned_int(ctx, 1, int_sort);

            Z3_mk_store(ctx, arr, zero, Z3_mk_le(ctx, lo, const_x));
            Z3_mk_store(ctx, arr, one, Z3_mk_le(ctx, const_x, hi));

            let and = Z3_mk_and(ctx, 2 as c_uint, &arr);

            let arr_bound = Z3_mk_array_sort(ctx, int_sort, int_sort);
            let arr_bound = Z3_sort_to_ast(ctx, arr_bound);

            Z3_mk_store(ctx, arr_bound, zero, const_x);
            // let arr_bound = Z3_get_as_array_func_decl(ctx, arr_bound);
            // let arr_bound = Z3_to_app(ctx, arr_bound);
            //let arr_bound = Z3_mk_app(ctx, arr_bound, 1, )
            //let x_bound = Z3_mk_bound(ctx, 0, int_sort);
            let int_sort_arr = Z3_mk_array_sort(ctx, int_sort, int_sort);
            Z3_mk_store(ctx, Z3_sort_to_ast(ctx, int_sort_arr), zero, const_x);

            // let names_arr = Z3_mk_array_sort(ctx, int_sort, Z3_mk_string_sort(ctx));
            // Z3_mk_store(
            //     ctx,
            //     Z3_sort_to_ast(ctx, names_arr),
            //     zero,
            //     Z3_mk_string_symbol(ctx, str_x.as_ptr()),
            // );
            return Z3_mk_lambda(
                ctx,
                1 as c_uint,
                &int_sort_arr,
                &Z3_mk_string_symbol(ctx, str_x.as_ptr()),
                Z3_mk_ite(
                    ctx,
                    and,
                    y,
                    Z3_mk_select(ctx, Z3_sort_to_ast(ctx, m), const_x),
                ),
            );
            // return Z3_mk_lambda_const(
            //     ctx,
            //     1,
            //     &arr_bound,
            //     Z3_mk_ite(
            //         ctx,
            //         and,
            //         y,
            //         Z3_mk_select(ctx, Z3_sort_to_ast(ctx, m), const_x),
            //     ),
            // );
        };

        let solver = Z3_mk_simple_solver(ctx);
        let one = Z3_mk_unsigned_int(ctx, 1, int_sort);
        let sev_oo = Z3_mk_unsigned_int(ctx, 700, int_sort);
        let six = Z3_mk_unsigned_int(ctx, 6, int_sort);
        let m1_ast = Z3_sort_to_ast(ctx, m1);

        println!("{:?}", memset(ctx, one, sev_oo, const_z, m));
        Z3_solver_assert(
            ctx,
            solver,
            Z3_mk_eq(ctx, m1_ast, memset(ctx, one, sev_oo, const_z, m)),
        );
        println!("{}", "hello");
        Z3_solver_assert(
            ctx,
            solver,
            Z3_mk_eq(ctx, Z3_mk_select(ctx, m1_ast, six), six),
        );

        assert_eq!(Z3_solver_check(ctx, solver), Z3_L_TRUE);
        let model = Z3_solver_get_model(ctx, solver);
        let model_s = Z3_model_to_string(ctx, model);

        let model_str = CStr::from_ptr(model_s).to_str().unwrap();

        println!("{}", model_str);
    }
}
