use postgresql::{self, Database, Postgres};
use ui::drawing::draw_result;

fn main() {
    let postgres = Postgres::connect("host=localhost user=postgres dbname=mydb password=123456");

    let postgres = postgres.unwrap();
    let result = postgres
        .execute_query_statement("select version();")
        .unwrap();

    draw_result(&result.cols, &result.rows);
}
