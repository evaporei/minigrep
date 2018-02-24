use super::super::*;

#[test]
fn config_new_should_return_config_when_not_var() {
    let args = vec![
        "program_name".to_string(),
        "bla".to_string(),
        "my_file.txt".to_string(),
    ];

    let config = Config::new(&args)
        .expect("to create new instance of Config");

    assert_eq!(config, Config {
        query: "bla".to_string(),
        file_name: "my_file.txt".to_string(),
        case_sensitive: false,
    });
}

#[test]
fn config_new_should_return_config_when_has_var() {
    let args = vec![
        "program_name".to_string(),
        "bla".to_string(),
        "my_file.txt".to_string(),
    ];

    let config = Config::new(&args)
        .expect("to create new instance of Config");

    env::set_var("CASE_INSENSITIVE", "TRUE");

    assert_eq!(config, Config {
        query: "bla".to_string(),
        file_name: "my_file.txt".to_string(),
        case_sensitive: true,
    });
}

#[test]
#[should_panic(expected = "not enough arguments")]
fn config_new_should_return_error() {
    let args = vec![
        "program_name".to_string(),
        "bla".to_string(),
    ];

    Config::new(&args).unwrap();
}

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
