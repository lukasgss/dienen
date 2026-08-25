use postgresql::{Database, Postgres};
use ui::repl;

fn main() {
    let postgres =
        Postgres::connect("host=localhost user=postgres dbname=timeline password=123456")
            .expect("should be able to connect to the database");

    if let Err(err) = repl::run(postgres.as_ref()) {
        eprintln!("error: {err:?}");
    }
}
