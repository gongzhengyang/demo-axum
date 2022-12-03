use std::fmt::Debug;

use schemars::JsonSchema;
use serde::Serialize;

/// 列表查询返回
#[derive(Debug, Serialize, Default, JsonSchema)]
pub struct ListResponse {
    /// 最大页数
    pub max_page: u64,
    /// 结果列表
    pub results: Vec<entity::post::Model>,
}
