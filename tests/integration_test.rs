//! 集成测试示例
//! 
//! 集成测试用于测试整个应用程序的功能，包括多个模块的交互。

use std::process::Command;

/// 测试应用程序的基本功能
#[test]
fn test_app_basic_functionality() {
    // 构建应用程序
    let output = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Failed to build application");

    assert!(output.status.success(), "Build failed: {:?}", output);

    // 运行应用程序
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to run application");

    assert!(output.status.success(), "Application failed to run: {:?}", output);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("欢迎使用 Rust 模板项目"), "Unexpected output: {}", stdout);
}

/// 测试应用程序的版本信息
#[test]
fn test_app_version() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output();

    // 如果应用程序支持 --version 参数，则检查输出
    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(stdout.contains("0.1.0"), "Version not found in output: {}", stdout);
        }
    }
}

/// 测试应用程序的帮助信息
#[test]
fn test_app_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output();

    // 如果应用程序支持 --help 参数，则检查输出
    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(!stdout.is_empty(), "Help output should not be empty");
        }
    }
} 