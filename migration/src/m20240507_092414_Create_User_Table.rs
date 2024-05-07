use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "
                CREATE TABLE IF NOT EXISTS users(
                    id SERIAL PRIMARY KEY,
                    email VARCHAR UNIQUE NOT NULL,
                    first_name VARCHAR(255) NOT NULL,
                    last_name VARCHAR(255) NOT NULL,
                    password VARCHAR NOT NULL,
                    is_verified BOOLEAN DEFAULT FALSE NOT NULL,
                    is_deleted BOOLEAN DEFAULT FALSE NOT NULL,
                    created_at TIMESTAMP DEFAULT (TIMEZONE('utc', now())) NOT NULL,
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
            .execute_unprepared("DROP TABLE users")
            .await?;

        Ok(())
    }
}

