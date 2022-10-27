use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct Task {
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

#[tokio::main]
async fn main() -> Result<(), bool> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite:///Users/lele/Desktop/memo.db")
        .await
        .unwrap();

    // Task 必须实现FromRow trait
    let stream = sqlx::query_as::<_, Task>("select * from tasks where deleted = ?")
        .bind(0)
        .fetch_all(&pool)
        .await;

    println!("tasK: {:?}", stream);

    // query_as! 没有trait要求，但是要求有DATABASE_RUL env
    let res = sqlx::query_as!(Task, "select * from tasks where deleted = ?", 0u32)
        .fetch_all(&pool)
        .await;

    println!("tasK: {:?}", res);
    Ok(())
}
