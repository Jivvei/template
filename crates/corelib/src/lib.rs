/// 计算两个数的和
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 检查字符串是否为空
pub fn is_empty_string(s: &str) -> bool {
    s.trim().is_empty()
}

/// 改进的错误处理
pub fn process_data(data: &[i32]) -> Result<i32, String> {
    if data.is_empty() {
        return Err("数据不能为空".to_string());
    }
    let sum = data.iter().sum();
    Ok(sum)
}

/// 模式匹配分析字符串
pub fn analyze_value(value: &str) -> &'static str {
    match value {
        "" => "空字符串",
        s if s.len() < 5 => "短字符串",
        s if s.len() < 10 => "中等字符串",
        _ => "长字符串",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_is_empty_string() {
        assert!(is_empty_string(""));
        assert!(is_empty_string("   "));
        assert!(!is_empty_string("hello"));
        assert!(!is_empty_string("  hello  "));
    }

    #[test]
    fn test_process_data() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(process_data(&data), Ok(15));
        let empty_data = vec![];
        assert!(process_data(&empty_data).is_err());
    }

    #[test]
    fn test_analyze_value() {
        assert_eq!(analyze_value(""), "空字符串");
        assert_eq!(analyze_value("hi"), "短字符串");
        assert_eq!(analyze_value("hello"), "中等字符串");
        assert_eq!(analyze_value("very long string"), "长字符串");
    }
} 