pub use sea_orm_migration::prelude::*;

mod m20230202_071406_create_users_table;
mod m20230210_203800_create_transactions_table;
mod m20230211_100201_create_categories_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230202_071406_create_users_table::Migration),
            Box::new(m20230210_203800_create_transactions_table::Migration),
            Box::new(m20230211_100201_create_categories_table::Migration),
        ]
    }
}
