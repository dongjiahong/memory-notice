use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Tasks {
    id: u32,
    task: String,
    start_date: String,
    duration: u32,
    tip: String,
}

fn main() -> Result<()> {
    let db = Connection::open("/Users/lele/Desktop/memo.db")?;

    db.execute(
        "insert into tasks (task, start_date, duration, tip) values(?1, ?2, ?3, ?4)",
        ("任务222", "2022-10-17", 2, ""),
    )?;

    let mut tks = db.prepare("select * from tasks")?;
    let tasks_iter = tks.query_map([], |row| {
        Ok(Tasks {
            id: row.get(0)?,
            task: row.get(1)?,
            start_date: row.get(2)?,
            duration: row.get(3)?,
            tip: row.get(4)?,
        })
    })?;

    for t in tasks_iter {
        println!("task: {:?}", t.unwrap());
    }

    Ok(())
}
