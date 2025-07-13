# 贡献指南

感谢您对 Rust 项目模板的关注！我们欢迎所有形式的贡献。

## 🤝 如何贡献

### 报告 Bug

如果您发现了一个 bug，请：

1. 使用 GitHub Issues 搜索是否已经报告过
2. 如果没有，请创建一个新的 Issue
3. 使用 bug 报告模板，并提供以下信息：
   - 操作系统和 Rust 版本
   - 重现步骤
   - 期望行为和实际行为
   - 错误信息或截图

### 功能请求

如果您有新功能的想法，请：

1. 使用 GitHub Issues 搜索是否已经提出过
2. 如果没有，请创建一个新的 Issue
3. 使用功能请求模板，并详细描述：
   - 功能的具体用途
   - 实现建议
   - 使用场景

### 代码贡献

#### 开发环境设置

1. Fork 项目到您的 GitHub 账户
2. 克隆您的 fork 到本地：
   ```bash
   git clone https://github.com/Jivvei/template.git
   cd template
   ```
3. 安装开发工具：
   ```bash
   make install-tools
   ```
4. 安装 pre-commit 钩子：
   ```bash
   pre-commit install
   ```

#### 开发流程

1. 创建功能分支：
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. 进行开发并提交更改：
   ```bash
   # 遵循 Conventional Commits 规范
   git commit -m "feat: add new feature"
   git commit -m "fix: resolve bug in existing feature"
   git commit -m "docs: update documentation"
   ```

3. 运行测试和检查：
   ```bash
   make all
   ```

4. 推送分支：
   ```bash
   git push origin feature/your-feature-name
   ```

5. 创建 Pull Request

#### 代码规范

- **格式化**: 使用 `cargo fmt` 格式化代码
- **检查**: 使用 `cargo clippy` 检查代码质量
- **测试**: 为新功能添加测试
- **文档**: 更新相关文档
- **提交信息**: 遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范

#### 测试要求

- 所有新功能必须有测试覆盖
- 测试必须通过所有检查
- 集成测试应该覆盖主要功能
- 基准测试应该验证性能改进

### 文档贡献

文档贡献同样重要：

- 修复拼写错误
- 改进文档结构
- 添加使用示例
- 翻译文档
- 更新配置说明

## 📋 Pull Request 检查清单

在提交 Pull Request 之前，请确保：

- [ ] 代码遵循项目规范
- [ ] 所有测试通过
- [ ] 新功能有相应的测试
- [ ] 文档已更新
- [ ] 提交信息遵循规范
- [ ] 没有引入新的警告
- [ ] 性能没有显著下降

## 🏷️ 标签说明

我们使用以下标签来分类 Issues 和 Pull Requests：

- `bug`: Bug 报告
- `enhancement`: 功能增强
- `documentation`: 文档相关
- `good first issue`: 适合新贡献者
- `help wanted`: 需要帮助
- `question`: 问题讨论
- `wontfix`: 不会修复

## 🎯 开发重点

当前项目的开发重点：

1. **工具链完善**: 持续改进开发工具链
2. **文档质量**: 提供清晰、完整的文档
3. **示例丰富**: 添加更多实用的示例
4. **性能优化**: 优化构建和运行性能
5. **社区支持**: 改进社区贡献体验

## 📞 获取帮助

如果您在贡献过程中遇到问题：

1. 查看 [README.md](README.md) 中的文档
2. 搜索现有的 Issues 和 Discussions
3. 创建新的 Issue 寻求帮助
4. 联系项目维护者

## 🙏 致谢

感谢所有为项目做出贡献的开发者！

---

**让我们一起构建更好的 Rust 项目模板！ 🦀** 