use corelib::{add, analyze_value, is_empty_string, process_data};

fn main() {
    println!("欢迎使用 Rust 2024 模板项目！");

    // 演示基础函数
    let result = add(10, 20);
    println!("10 + 20 = {result}");

    let test_strings = vec!["", "   ", "hello", "  world  "];
    for s in test_strings {
        println!("字符串 '{s}' 是否为空: {}", is_empty_string(s));
    }

    // 演示新功能
    println!("\n--- 新功能演示 ---");

    // 数据处理
    let data = vec![1, 2, 3, 4, 5];
    match process_data(&data) {
        Ok(sum) => println!("数据总和: {sum}"),
        Err(e) => println!("处理错误: {e}"),
    }

    // 字符串分析
    let test_values = vec!["", "hi", "hello", "very long string here"];
    for value in test_values {
        println!("'{value}' -> {}", analyze_value(value));
    }

    println!("\n项目运行成功！");
}
