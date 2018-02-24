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

#[test]
fn search_should_return_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}

#[test]
fn search_should_return_no_result() {
    let query = "not in content";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let results: Vec<&str> = Vec::new();

    assert_eq!(
        results,
        search(query, contents)
    );
}
