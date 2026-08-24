use std::io::{self, Write, stdout};

pub fn ask_input(database_name: &str) -> String {
    print!("{}> ", database_name);

    let _ = stdout().flush();

    let mut input_query = String::new();
    io::stdin()
        .read_line(&mut input_query)
        .expect("should be able to parse input query");

    input_query
}
