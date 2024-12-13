use derive_more::derive::Display;
use sea_orm::DbErr;

#[derive(PartialEq, Eq, Display)]
pub enum ServiceError{
    NotWork,
    Database(DbErr),
}