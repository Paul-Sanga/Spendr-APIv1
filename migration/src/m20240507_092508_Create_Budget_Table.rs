use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "
                CREATE TABLE IF NOT EXISTS budget(
                    id SERIAL PRIMARY KEY,
                    user_id INTEGER REFERENCES users(id) NOT NULL,
                    category VARCHAR NOT NULL,
                    amount DECIMAL NOT NULL,
                    created_at TIMESTAMP NOT NULL DEFAULT (TIMEZONE('utc', now())),
                    updated_at TIMESTAMP
                )
            "
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE budget")
            .await?;
        Ok(())
    }
}

