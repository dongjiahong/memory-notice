use std::{cmp::Ordering};

use crate::sm2::Item;
use chrono::{Duration, Local, TimeZone};
use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::State;
use tracing::debug;

#[derive(Serialize, Default, Clone, Debug)]
pub struct Task {
    id: i64,
    task: String,        // 任务名称
    last_date: String,   // 上次任务开始的时间
    review_date: String, // 下次review的时间
    duration: i64,       // 任务需要用时
    repetitions: i64,    // 复习次数
    efactor: f64,        // 影响因子
    tip: String,         // 备注
    deleted: i64,        // 默认为0，如果删除就填入时间戳
}

pub struct DbConnection {
    db:Pool<Sqlite>,
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
    pub async fn default() -> Self {
        let db_file = "/Users/lele/Desktop/memo.db";
        debug!("open sqlite db file: {}", db_file);
        let db = SqlitePoolOptions::new()
            .connect(format!("sqlite://{}", db_file).as_str())
            .await
            .unwrap();

        return DbConnection {db};
    }
}

fn wrap_response<T>(status: ResponseStatus, msg: String, info: Option<T>) -> Result<Response<T>,()> {
    return Ok(Response { status, msg, info });
}

#[tauri::command]
pub async fn add_task(
    task: String,
    start: String,
    duration: u32,
    tip: String,
    connection: State<'_, DbConnection>,
) -> Result<Response<String>,()> {
    let res = sqlx::query!(
        "insert into tasks (task, last_date, review_date, duration, repetitions, tip) values(?, ?, ?, ?, ?, ?)",
        task, "", start, duration, 1, tip)
        .execute(&connection.db)
        .await;

    match res {
        Ok(_) => wrap_response(ResponseStatus::Success, "success".into(), None),
        Err(e) => wrap_response(ResponseStatus::Failed, e.to_string(), None),
    }
}

#[tauri::command]
pub async fn get_tasks(connection: State<'_, DbConnection>) -> Result<Response<FetchTask>,()> {
    let tasks = sqlx::query_as!(Task, "select * from tasks")
        .fetch_all(&connection.db)
        .await
        .unwrap();

    let mut fetch = FetchTask::default();
    let now = Local::now();
    for t in tasks {
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
pub async fn get_task_by_id(id: u32, connection: State<'_, DbConnection>) -> Result<Response<Task>, ()> {
    let task = sqlx::query_as!(Task, "select * from tasks where id = ?", id)
        .fetch_optional(&connection.db)
        .await;

    match task.unwrap() {
        Some(t) => {
            return wrap_response(
                ResponseStatus::Success,
                "success".into(),
                Some(t),
            );
        }
        None => return wrap_response(ResponseStatus::Success, "success".into(), None)
    }
}

/*
   参数：quatily:
           2: 记不起来
           3: 比较难
           4: 一般
           5: 没问题
   返回: 下次复习的时间
*/
#[tauri::command]
pub async fn review_task(
    id: u32,
    last: String,
    repetitions: usize,
    efactor: f64,
    quality: u8,
    connection: State<'_, DbConnection>,
) -> Result<Response<String>,()> {
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

    let review_date_str = review_date.format("%Y-%m-%d").to_string();
    let new_item_repetition = new_item.repetitions() as i64;
    let new_item_efactor = new_item.efactor();
    let res = sqlx::query_as!(Task, 
        "update tasks set last_date = ?, review_date = ?, repetitions = ?, efactor = ? where id = ?",
            last, review_date_str, new_item_repetition, new_item_efactor, id)
            .execute(&connection.db)
            .await;

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
