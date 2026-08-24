use postgresql::{self, Database, Postgres};
use ui::{drawing::draw_result, input::ask_input};

fn main() {
    let postgres =
        Postgres::connect("host=localhost user=postgres dbname=timeline password=123456");

    let postgres = postgres.unwrap();

    loop {
        let query = ask_input(postgres.database_name());

        let query_result = postgres.execute_query_statement(&query);

        match query_result {
            Ok(result) => {
                if let Err(drawing_result) = draw_result(&result.cols, &result.rows) {
                    println!("{}", drawing_result);
                }
            }
            Err(_) => println!("{}", postgres.error_message()),
        }
    }
}
