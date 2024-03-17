use puchiprop_core::*;

pub fn report_error(_testname: &str, err: &TestErrorReport) {
    eprintln!("---- test case ----");
    eprintln!("{}", err.case);

    eprintln!("---- test state ----");
    eprintln!("{}", err.state);
}
