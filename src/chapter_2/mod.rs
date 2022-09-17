mod equality_as_second_order;
mod expr_access;
mod memset;
mod quantifiers;
mod rust2smtlib;
mod smt101;
mod sorts_are_nonempty;
mod terms_and_formulas;
mod tieshirt;

pub fn tieshirt() {
    tieshirt::run();
}

pub fn smt101() {
    smt101::run();
}

pub fn rust2smtlib() {
    rust2smtlib::run();
}

pub fn sorts_are_nonempty() {
    sorts_are_nonempty::run();
}

pub fn terms_and_formulas() {
    terms_and_formulas::run();
}

pub fn expr_access() {
    expr_access::run();
}

pub fn quantifiers() {
    quantifiers::run();
}

pub fn memset() {
    memset::run();
}

pub fn equality_as_second_order() {
    equality_as_second_order::run();
}
