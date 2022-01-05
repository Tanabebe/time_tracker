use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use r2d2::Pool;

pub struct DataContext {
    _pool: Pool<ConnectionManager<PgConnection>>,
}

impl DataContext {
    pub fn new() -> DataContext {
        dotenv::dotenv().ok();
        std::env::set_var("RUST_LOG", "actix_web=debug");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URLを設定してください");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Poolの作成に失敗しました");
        DataContext { _pool : pool }
    }
}