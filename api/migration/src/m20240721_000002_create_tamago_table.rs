use sea_orm_migration::prelude::*;

use crate::m20240721_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20240721_000002_create_tamago_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the User table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tamago::Table)
                    .col(
                        ColumnDef::new(Tamago::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tamago::UserId).integer().not_null())
                    .col(ColumnDef::new(Tamago::Name).string().not_null())
                    .col(ColumnDef::new(Tamago::Health).integer().not_null())
                    .col(ColumnDef::new(Tamago::HealthMax).integer().not_null())
                    .col(ColumnDef::new(Tamago::Evolution).integer().default(0))
                    .col(ColumnDef::new(Tamago::Level).integer().default(1))
                    .col(
                        ColumnDef::new(Tamago::RegenerationRate)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tamago::RegenerationIntervalMinute)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tamago::IsDead).boolean().default(false))
                    .col(ColumnDef::new(Tamago::DeathTypeId).integer().not_null())
                    .col(ColumnDef::new(Tamago::DeathDate).timestamp().null())
                    .col(
                        ColumnDef::new(Tamago::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-tamago-user_id")
                            .from(Tamago::Table, Tamago::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Tamago table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tamago::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Tamago {
    Table,
    Id,
    UserId,
    Name,
    Health,
    HealthMax,
    Evolution,
    Level,
    RegenerationRate,
    RegenerationIntervalMinute,
    IsDead,
    DeathTypeId,
    DeathDate,
    CreatedAt,
}
