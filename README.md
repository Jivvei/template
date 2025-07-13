# Rust 2024 项目模板

一个现代化的 Rust 2024 项目模板，集成了完整的开发工具链和最佳实践，适合快速启动生产级别的 Rust 项目。

## ✨ 特性

- 🚀 **现代化工具链**: 集成 cargo-generate、cargo-deny、typos、git-cliff 等工具
- 🦀 **Rust 2024**: 使用最新的 Rust 2024 edition 和语言特性
- 🔒 **安全优先**: 自动安全漏洞检查、许可证合规性验证
- 📝 **代码质量**: 自动格式化、linting、拼写检查
- 🧪 **测试友好**: 集成 cargo-nextest 增强测试工具
- 🔄 **CI/CD 就绪**: 完整的 GitHub Actions 工作流
- 📚 **文档完善**: 自动生成 changelog、代码文档
- 🛠️ **开发体验**: VS Code 配置、Makefile 命令简化

## 🏗️ 项目结构

```
template/
├── src/
│   └── main.rs              # 主程序入口
├── assets/                  # 静态资源
│   ├── README.md
│   └── juventus.csv         # 示例数据
├── .github/workflows/       # CI/CD 配置
│   └── build.yml
├── .cargo/
│   └── config.toml          # Cargo 配置
├── .vscode/                 # VS Code 配置
├── docs/                    # 项目文档
├── tests/                   # 集成测试
├── examples/                # 示例代码
├── benches/                 # 基准测试
├── Cargo.toml               # 项目配置
├── Cargo.lock               # 依赖锁定
├── Makefile                 # 构建脚本
├── rustfmt.toml             # 代码格式化配置
├── .clippy.toml             # 代码检查配置
├── deny.toml                # 依赖安全检查配置
├── cliff.toml               # Changelog 生成配置
├── _typos.toml              # 拼写检查配置
├── .pre-commit-config.yaml  # Git 钩子配置
├── .gitignore               # Git 忽略文件
└── README.md                # 项目文档
```

## 🚀 快速开始

### 1. 环境准备

#### 安装 Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 安装开发工具
```bash
# 使用 Makefile 一键安装所有工具
make install-tools

# 或手动安装
cargo install cargo-generate
cargo install --locked cargo-deny
cargo install typos-cli
cargo install git-cliff
cargo install cargo-nextest --locked
pipx install pre-commit
```

#### 安装 VS Code 插件
推荐安装以下插件以获得最佳开发体验：

- **rust-analyzer**: Rust 语言支持
- **crates**: Rust 包管理
- **Even Better TOML**: TOML 文件支持
- **Better Comments**: 优化注释显示
- **Error Lens**: 错误提示优化
- **GitLens**: Git 增强
- **Github Copilot**: 代码提示
- **indent-rainbow**: 缩进显示优化
- **Prettier**: 代码格式化
- **REST client**: REST API 调试
- **Rust Test lens**: Rust 测试支持
- **Rust Test Explorer**: Rust 测试概览
- **TODO Highlight**: TODO 高亮
- **vscode-icons**: 图标优化
- **YAML**: YAML 文件支持

### 2. 使用模板

#### 从模板创建新项目
```bash
cargo generate Jivvei/template
```

#### 或克隆现有项目
```bash
git clone https://github.com/Jivvei/template.git
cd template
```

### 3. 初始化项目
```bash
# 安装 pre-commit 钩子
pre-commit install

# 运行所有检查确保环境正常
make all
```

## 🛠️ 开发指南

### 常用命令

使用 Makefile 简化开发流程：

```bash
# 查看所有可用命令
make help

# 构建项目
make build

# 运行测试
make test

# 格式化代码
make fmt

# 运行代码检查
make clippy

# 运行所有检查
make check

# 构建、测试和检查
make all

# 清理构建文件
make clean
```

### 使用 Cargo 别名

项目配置了常用的 Cargo 别名：

```bash
cargo b    # build
cargo t    # test
cargo c    # check
cargo f    # fmt
cargo r    # run
cargo cl   # clippy
cargo d    # doc
cargo u    # update
```

### 代码规范

#### 提交规范
项目使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```bash
feat: 添加新功能
fix: 修复 bug
docs: 更新文档
style: 代码格式调整
refactor: 代码重构
test: 添加测试
chore: 构建过程或辅助工具的变动
```

#### 代码格式化
```bash
# 自动格式化
cargo fmt

# 检查格式
cargo fmt -- --check
```

#### 代码检查
```bash
# 运行 clippy
cargo clippy --all-targets --all-features --tests --benches -- -D warnings

# 运行 cargo-deny 检查
cargo deny check
```

### 测试

#### 单元测试
```bash
# 运行所有测试
cargo test

# 使用 nextest 运行测试
cargo nextest run --all-features
```

#### 集成测试
在 `tests/` 目录下创建集成测试文件。

#### 基准测试
在 `benches/` 目录下创建基准测试文件。

### 文档

#### 生成文档
```bash
# 生成并打开文档
cargo doc --open

# 生成文档但不打开
cargo doc
```

#### 文档注释规范
```rust
/// 函数的功能描述
///
/// # 参数
/// * `param1` - 参数1的描述
/// * `param2` - 参数2的描述
///
/// # 返回值
/// 返回值的描述
///
/// # 示例
/// ```
/// use crate::function_name;
/// let result = function_name(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn function_name(param1: i32, param2: i32) -> i32 {
    param1 + param2
}
```

## 🔧 配置说明

### Cargo 配置 (`.cargo/config.toml`)
- 设置构建目标和链接器
- 配置常用命令别名

### Rustfmt 配置 (`rustfmt.toml`)
- 代码格式化规则
- 导入排序设置
- 代码风格统一

### Clippy 配置 (`.clippy.toml`)
- 代码检查规则
- 性能优化建议
- 正确性检查

### Cargo-deny 配置 (`deny.toml`)
- 安全漏洞检查
- 许可证合规性
- 依赖版本管理

### Git-cliff 配置 (`cliff.toml`)
- Changelog 生成规则
- 提交信息解析
- 版本标签管理

## 🚀 CI/CD

项目包含完整的 GitHub Actions 工作流：

- **代码质量检查**: 格式化、linting、拼写检查
- **安全检查**: 依赖漏洞扫描
- **测试**: 单元测试、集成测试
- **文档生成**: 自动生成 changelog
- **发布**: 自动发布到 GitHub Releases

## 📦 发布流程

### 1. 版本管理
```bash
# 更新版本号
cargo set-version <new-version>

# 或手动编辑 Cargo.toml
```

### 2. 发布前检查
```bash
make release-check
```

### 3. 创建发布
```bash
# 创建标签
git tag v1.0.0

# 推送标签
git push origin v1.0.0
```

## 🤝 贡献指南

### 开发流程
1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码审查
- 所有代码变更需要经过审查
- 确保所有测试通过
- 遵循项目的代码规范

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Rust 官方团队](https://www.rust-lang.org/)
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)
- [git-cliff](https://github.com/orhun/git-cliff)
- [typos](https://github.com/crate-ci/typos)
- [cargo-nextest](https://github.com/nextest-rs/nextest)

## 📞 支持

如果您在使用过程中遇到问题，请：

1. 查看 [Issues](https://github.com/Jivvei/template/issues) 是否有类似问题
2. 创建新的 Issue 描述问题
3. 联系项目维护者

---

**Happy Coding! 🦀**
