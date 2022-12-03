use serde::Deserialize;

/// 查询参数
#[derive(Deserialize, Debug)]
pub struct Params {
    /// 页码
    #[serde(default = "default_page_num")]
    pub page_num: u64,
    /// 每页的大小
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page_num() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_params_default() {
        let json = r#"{}"#;
        let struct_value: Params = serde_json::from_str(json).unwrap();
        assert_eq!(struct_value.page_num, 1);
        assert_eq!(struct_value.page_size, 10);
    }
}
