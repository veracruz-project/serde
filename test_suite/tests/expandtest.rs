#[cfg_attr(target_os = "emscripten", ignore)]
#[cfg_attr(not(expandtest), ignore)]
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn expandtest() {
    macrotest::expand("tests/expand/*.rs");
}
