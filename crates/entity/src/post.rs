//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.5

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// 文章模型
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize, JsonSchema)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    /// 标题
    pub title: String,
    /// 文本
    pub text: String,
    /// 是否被检查过
    pub is_checked: bool,
    /// 作者名字
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
