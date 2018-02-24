use super::super::*;

#[test]
fn run_should_return_ok() {
    let args = vec![
        "minigrep".to_string(),
        "bla".to_string(),
        "poem.txt".to_string(),
    ];

    let config = Config::new(&args).unwrap();
    run(config).expect("to run without returning an Error");
}

#[test]
#[should_panic]
fn run_should_return_error() {
    let args = vec![
        "minigrep".to_string(),
        "bla".to_string(),
        "non_exiting_file.txt".to_string(),
    ];

    let config = Config::new(&args).unwrap();
    run(config).unwrap();
}
