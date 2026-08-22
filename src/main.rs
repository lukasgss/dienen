use postgresql::{self, Database, Postgres};
use ui::drawing::draw_result;

fn main() {
    let postgres =
        Postgres::connect("host=localhost user=postgres dbname=timeline password=123456");

    let postgres = postgres.unwrap();
    let result = postgres
        .execute_query_statement("select * from \"Plan\";")
        .unwrap();

    draw_result(&result.cols, &result.rows);
}
