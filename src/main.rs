#![allow(dead_code)]
mod chapter_2;
mod chapter_3;
fn main() {
    // Chapter 2 - Logical Interfaces to Z3.
    // ------------------------------------------------
    // chapter_2::tieshirt();
    // chapter_2::smt101();
    // chapter_2::rust2smtlib();

    // 2.1 Sorts
    // ------------------------
    // chapter_2::sorts_are_nonempty();

    // 2.3 Terms and Formulas
    // ------------------------
    // chapter_2::terms_and_formulas();
    // chapter_2::expr_access();

    // 2.4 Quantifiers and Lambda binding
    // ------------------------
    // chapter_2::quantifiers();
    // chapter_2::memset(); FIX Needed
    // chapter_2::equality_as_second_order();

    // Chapter 3 - Theories.
    // ------------------------------------------------

    // 3.1. EUF: Equality and Uninterpreted Functions
    // ------------------------
    // chapter_3::euf();

    // 3.1.2 EUF models
    // ------------------------
    // chapter_3::euf_fg();

    // 3.2 Arithmetic
    // ------------------------

    // 3.2.1. Solving LRA: Linear Real Arithmetic
    // ------------------------
    // chapter_3::qflra();

    // 3.3 Arrays
    // ------------------------
    // chapter_3::arrays();

    // 3.4 Bit-Vectors
    // ------------------------
    // chapter_3::bv1();
    // chapter_3::bv2();

    // 3.4.2 Floating Point Arithmetic
    // ------------------------
    // chapter_3::fp();

    // 3.5 Algebraic Datatypes
    // ------------------------
    // chapter_3::qfdt();

    // 3.5 Sequences and Strings
    // ------------------------
    chapter_3::seq1();
}
