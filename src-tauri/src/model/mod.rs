use std::{cmp::Ordering, sync::Mutex};

use crate::sm2::Item;
use chrono::{Duration, Local, TimeZone};
use rusqlite::Connection;
use serde::Serialize;
use tauri::State;
use tracing::debug;

#[derive(Serialize, Default, Clone, Debug)]
pub struct Task {
    id: u32,
    task: String,        // 任务名称
    last_date: String,   // 上次任务开始的时间
    review_date: String, // 下次review的时间
    duration: u32,       // 任务需要用时
    repetitions: u32,    // 复习次数
    efactor: f64,        // 影响因子
    tip: String,         // 备注
    deleted: u32,        // 默认为0，如果删除就填入时间戳
}

pub struct DbConnection {
    db: Mutex<Connection>,
}

#[derive(Serialize)]
pub enum ResponseStatus {
    Success,
    Failed,
}

#[derive(Serialize)]
pub struct Response<T> {
    status: ResponseStatus,
    msg: String,
    info: Option<T>,
}

#[derive(Serialize, Default, Debug)]
pub struct FetchTask {
    delay: Vec<Task>,
    today: Vec<Task>,
    week: Vec<Task>,
}

impl FetchTask {
    fn default() -> Self {
        FetchTask {
            delay: vec![],
            today: vec![],
            week: vec![],
        }
    }
}

impl DbConnection {
    pub fn default() -> Self {
        let db_file = "/Users/lele/Desktop/memo.db";
        debug!("open sqlite db file: {}", db_file);
        let db = Connection::open(db_file).unwrap();

        return DbConnection { db: Mutex::new(db) };
    }
}

fn wrap_response<T>(status: ResponseStatus, msg: String, info: Option<T>) -> Response<T> {
    return Response { status, msg, info };
}

#[tauri::command]
pub fn add_task(
    task: String,
    start: String,
    duration: u32,
    tip: String,
    connection: State<'_, DbConnection>,
) -> Response<String> {
    let res = connection.db.lock().unwrap().execute(
        "insert into tasks (task, last_date, review_date, duration,repetitions, tip) values(?1, ?2, ?3, ?4, ?5, ?6)",
        (task.as_str(), "", start.as_str(), duration, 1, tip.as_str()),
    );

    match res {
        Ok(_) => wrap_response(ResponseStatus::Success, "success".into(), None),
        Err(e) => wrap_response(ResponseStatus::Failed, e.to_string(), None),
    }
}

#[tauri::command]
pub fn get_tasks(connect: State<'_, DbConnection>) -> Response<FetchTask> {
    let b = connect.db.lock().unwrap();
    let mut binding = b.prepare("select * from tasks").unwrap();

    let tks_iter = binding.query_map([], |row| {
        Ok(Task {
            id: row.get(0).unwrap(),
            task: row.get(1).unwrap(),
            last_date: row.get(2).unwrap(),
            review_date: row.get(3).unwrap(),
            duration: row.get(4).unwrap(),
            repetitions: row.get(5).unwrap(),
            efactor: row.get(6).unwrap(),
            tip: row.get(7).unwrap(),
            deleted: row.get(8).unwrap(),
        })
    });

    let mut fetch = FetchTask::default();
    let now = Local::now();
    for t in tks_iter.unwrap() {
        let t = t.unwrap();

        let task_review_date = Local
            .datetime_from_str(
                format!("{} 00:00:00", t.review_date).as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap();
        let date = task_review_date + Duration::days(t.duration as i64);
        let week_date = now + Duration::days(7i64);

        debug!(
            "now: {}, task_review: {}, date: {}, week_date: {}",
            now, task_review_date, date, week_date
        );
        if date.cmp(&now) == Ordering::Less {
            fetch.delay.push(t);
            continue;
        }
        if task_review_date.cmp(&now) == Ordering::Greater
            && task_review_date.cmp(&week_date) == Ordering::Less
        {
            fetch.week.push(t.clone());
        }
        if date.cmp(&now) == Ordering::Greater && task_review_date.cmp(&now) == Ordering::Less {
            fetch.today.push(t);
        }
    }

    debug!("---> fetch: {:?}", fetch);

    return wrap_response(ResponseStatus::Success, "success".into(), Some(fetch));
}

#[tauri::command]
/*
   参数：quatily:
           2: 记不起来
           3: 比较难
           4: 一般
           5: 没问题
   返回: 下次复习的时间
*/
pub fn review_task(
    id: u32,
    last: String,
    repetitions: usize,
    efactor: f64,
    quality: u8,
    connection: State<'_, DbConnection>,
) -> Response<String> {
    // 1. 获取当前日期
    let now = Local::now();
    // 2. 获取下次复习日期
    let item = Item::new(repetitions, efactor);
    let new_item = item.review(quality).unwrap();
    let interval = new_item.interval(); // 间隔时间
    let review_date = now + Duration::days(interval as i64);

    debug!(
        "---->id: {}, last: {}, repetitions: {}, efactor: {}, qualitiy: {}",
        id, last, repetitions, efactor, quality
    );

    let res = connection.db.lock().unwrap().execute(
        "update tasks set last_date = ?1, review_date = ?2, repetitions = ?3, efactor = ?4 where id = ?5",
        (
            last,
            review_date.format("%Y-%m-%d").to_string(),
            new_item.repetitions(),
            new_item.efactor(),
            id,
        ),
    );

    match res {
        Ok(_) => wrap_response(ResponseStatus::Success, "success".into(), None),
        Err(e) => wrap_response(ResponseStatus::Failed, e.to_string(), None),
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;
    use chrono::TimeZone;

    #[test]
    fn test_local_date() {
        assert_eq!(
            Local
                .datetime_from_str("2022-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .format("%Y-%m-%d")
                .to_string(),
            "2022-01-01"
        );
    }
}
