pub mod functions;

#[test]
fn fails_and_panics() {
    googletest::assert_that!(functions::sum_of_a(5), googletest::prelude::eq(10));
}

#[googletest::test]
fn two_logged_failures() {
    googletest::expect_that!(functions::sum_of_a(5), googletest::prelude::eq(10));
}

#[test]
fn fails_immediately_without_panic() -> googletest::Result<()> {
    googletest::verify_that!(functions::sum_of_a(5), googletest::prelude::eq(10))?;
    Ok(())
}

/*
#[test]
fn simple_assertion() -> googletest::Result<()> {
    googletest::expect_that!(
        functions::arr_of_b(10),
        googletest::prelude::contains(googletest::prelude::ge(0))
    );
    Ok(())
}
*/
