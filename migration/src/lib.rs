pub use sea_orm_migration::prelude::*;

#[allow(non_snake_case)]
mod m20240507_092414_Create_User_Table;
#[allow(non_snake_case)]
mod m20240507_092448_Create_Transaction_Table;
#[allow(non_snake_case)]
mod m20240507_092508_Create_Budget_Table;
#[allow(non_snake_case)]
mod m20240507_092521_Create_Tip_Table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240507_092414_Create_User_Table::Migration),
            Box::new(m20240507_092448_Create_Transaction_Table::Migration),
            Box::new(m20240507_092508_Create_Budget_Table::Migration),
            Box::new(m20240507_092521_Create_Tip_Table::Migration),
        ]
    }
}
