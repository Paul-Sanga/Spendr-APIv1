use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "
            CREATE TYPE expense_type AS ENUM ('income', 'expense');
                CREATE TABLE IF NOT EXISTS transactions(
                    id SERIAL PRIMARY KEY,
                    user_id INTEGER  NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
                    balance NUMERIC NOT NULL,
                    category CHARACTER VARYING NOT NULL,
                    type expense_type NOT NULL,
                    date DATE NOT NULL
                );
            "
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .get_connection()
            .execute_unprepared("DROP TABLE transactions")
            .await?;

        Ok(())
    }
}


