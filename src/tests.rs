use super::*;

#[test]
fn config_new_should_return_config() {
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
