use postgresql::{self, Database, Postgres};

fn main() {
    let postgres = Postgres::connect("host=localhost user=postgres dbname=mydb password=123456");

    let postgres = postgres.unwrap();
    let result = postgres.execute_query_statement("select version();");
    println!("{:?}", result);
}
