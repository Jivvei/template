# Rust 项目 Makefile
# 提供常用的开发命令

.PHONY: help build test clean fmt clippy deny check all install-tools

# 默认目标
help:
	@echo "可用的命令："
	@echo "  build     - 构建项目"
	@echo "  test      - 运行测试"
	@echo "  clean     - 清理构建文件"
	@echo "  fmt       - 格式化代码"
	@echo "  clippy    - 运行 clippy 检查"
	@echo "  deny      - 运行 cargo-deny 检查"
	@echo "  check     - 运行所有检查"
	@echo "  all       - 构建、测试和检查"
	@echo "  install-tools - 安装开发工具"

# 构建项目
build:
	cargo build

# 构建发布版本
build-release:
	cargo build --release

# 运行测试
test:
	cargo test

# 运行测试（使用 nextest）
test-nextest:
	cargo nextest run --all-features

# 清理构建文件
clean:
	cargo clean

# 格式化代码
fmt:
	cargo fmt

# 检查代码格式
fmt-check:
	cargo fmt -- --check

# 运行 clippy 检查
clippy:
	cargo clippy --all-targets --all-features --tests --benches -- -D warnings

# 运行 cargo-deny 检查
deny:
	cargo deny check

# 运行拼写检查
typos:
	typos

# 运行所有检查
check: fmt-check clippy
	@echo "所有检查完成！"

# 构建、测试和检查
all: build test check
	@echo "构建、测试和检查完成！"

# 安装开发工具
install-tools:
	@echo "安装开发工具..."
	cargo install cargo-generate
	cargo install --locked cargo-deny
	cargo install typos-cli
	cargo install git-cliff
	cargo install cargo-nextest --locked
	@echo "开发工具安装完成！"

# 运行基准测试
bench:
	cargo bench

# 生成文档
doc:
	cargo doc --open

# 检查依赖更新
update:
	cargo update

# 运行示例
example:
	cargo run --example

# 发布前检查
release-check: clean build-release test clippy deny
	@echo "发布前检查完成！"

# 生成覆盖率报告
coverage:
	cargo llvm-cov --html

# 检查依赖更新
outdated:
	cargo outdated

# 安全审计
audit:
	cargo audit

# 清理所有生成的文件
clean-all: clean
	rm -rf target/
	rm -rf docs/
	rm -rf coverage/
	@echo "所有生成的文件已清理！"

# 完整检查（包括安全审计）
full-check: check audit outdated
	@echo "完整检查完成！"

# 开发环境初始化
init: install-tools
	pre-commit install
	@echo "开发环境初始化完成！"

# 帮助信息（详细版）
help-detailed:
	@echo "详细命令说明："
	@echo ""
	@echo "构建相关："
	@echo "  build          - 构建项目"
	@echo "  build-release  - 构建发布版本"
	@echo "  clean          - 清理构建文件"
	@echo "  clean-all      - 清理所有生成的文件"
	@echo ""
	@echo "测试相关："
	@echo "  test           - 运行单元测试"
	@echo "  test-nextest   - 使用 nextest 运行测试"
	@echo "  bench          - 运行基准测试"
	@echo "  coverage       - 生成覆盖率报告"
	@echo ""
	@echo "代码质量："
	@echo "  fmt            - 格式化代码"
	@echo "  fmt-check      - 检查代码格式"
	@echo "  clippy         - 运行 clippy 检查"
	@echo "  deny           - 运行 cargo-deny 检查"
	@echo "  typos          - 运行拼写检查"
	@echo "  check          - 运行所有代码检查"
	@echo "  full-check     - 运行完整检查（包括安全审计）"
	@echo ""
	@echo "文档相关："
	@echo "  doc            - 生成并打开文档"
	@echo "  update         - 检查依赖更新"
	@echo "  outdated       - 显示过时的依赖"
	@echo "  audit          - 安全审计"
	@echo ""
	@echo "工具安装："
	@echo "  install-tools  - 安装开发工具"
	@echo "  init           - 初始化开发环境"
	@echo ""
	@echo "发布相关："
	@echo "  release-check  - 发布前检查"
	@echo "  all            - 构建、测试和检查" 