#[macro_use]
extern crate eyre;

use eyre::Result;

use rusqlite::*;

fn main() -> Result<()> {
    let conn = Connection::open("./tests/test.db3")?;

    let mut stmt = conn.prepare(
        "SELECT id, name, json_extract(data, '$.age') AS age,\
         json_extract(data, '$.email') AS email FROM users;",
    )?;
    let rows = &mut stmt.query(())?;
    while let Some(row) = rows.next()? {
        dbg!(row);
    }

    Ok(())
}
