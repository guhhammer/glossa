use glossa::prelude::*;

// =====================================================
// CORE FUNCTIONS
// =====================================================
fn sum(a: u8, b: u8) -> u8 { a + b }
fn sum2(a: u8, b: u8, c: u8) -> u8 { a + b + c }
fn double(x: u8) -> u8 { x * 2 }
fn add(a: u8, b: u8) -> u8 { a + b }
fn print(_x: u8) { /* side-effect only */ }

// =====================================================
// GLASSA DSL TESTS
// =====================================================
#[test]
fn test_glossa_dsl() {
    // -------------------------------------------------
    // Auto-print behavior
    // -------------------------------------------------
    #[glossa]
    fn auto_print() {
        "this prints automatically!";
        let x = "asa";
        "this formats a value too, like x = {x}";
    }
    auto_print();

    // -------------------------------------------------
    // Basic pipeline
    // -------------------------------------------------
    #[glossa]
    fn pipeline_basic() {
        let a = "10,20" >> sum >> double;
        assert_eq!(a, 60);
    }
    pipeline_basic();

    // -------------------------------------------------
    // Inline execution pipeline
    // -------------------------------------------------
    #[glossa]
    fn pipeline_inline() {
        "10,20" >> sum >> double >> print;
    }
    pipeline_inline();

    // -------------------------------------------------
    // Partial application tests
    // -------------------------------------------------
    #[glossa]
    fn partial_application() {
        "10,20" >> sum >> add(5) >> print;
        "10,20" >> sum >> add(5, __) >> print;
        "10,20" >> sum >> add(__, 5) >> print;
    }
    partial_application();

    // -------------------------------------------------
    // Closure in pipeline
    // -------------------------------------------------
    #[glossa]
    fn closure_pipeline() {
        "10,20" >> sum >> (|x| x * 2) >> print;
    }
    closure_pipeline();

    // -------------------------------------------------
    // compose!
    // -------------------------------------------------
    #[glossa]
    fn compose_test() {
        let f1 = compose!(add(5), double);
        assert_eq!(f1(10), 30);
    }
    compose_test();

    // -------------------------------------------------
    // pipe!
    // -------------------------------------------------
    #[glossa]
    fn pipe_test() {
        let f2 = pipe!(|a, b, c| sum2 >> double);
        assert_eq!(f2(10, 20, 30), 120);
    }
    pipe_test();

    // -------------------------------------------------
    // glossa_fn!
    // -------------------------------------------------
    #[glossa]
    fn glossa_fn_test() {
        let f3 = glossa_fn!(|x, y| add >> double >> print);
        f3(10, 20);
    }
    glossa_fn_test();
}