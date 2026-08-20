use postgresql::{self, Database, Postgres};

fn main() {
    let postgres = Postgres::connect("host=localhost user=postgres dbname=mydb password=123456");

    println!("{:?}", postgres);
}
