use napi_derive_ohos::napi;
use napi_ohos::Result;
use rusqlite::Connection;

#[napi(object)]
pub struct Person {
    pub id: i32,
    pub name: String,
}

#[napi]
pub fn add() -> Result<Vec<Person>> {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS persons (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .unwrap();

    conn.execute(
        "INSERT INTO persons (name) VALUES (?1), (?2), (?3)",
        ["Steven", "John", "Alex"].map(|n| n.to_string()),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, name FROM persons").unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .unwrap();

    let mut v: Vec<Person> = Vec::new();

    for person in rows {
        match person {
            Ok(p) => v.push(p),
            Err(e) => {}
        }
    }

    Ok(v)
}
