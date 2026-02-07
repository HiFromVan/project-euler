# Project Euler Solutions

本仓库收录了 [Project Euler](https://projecteuler.net/) 题目的求解代码与数学思路分析。

## 项目简介

Project Euler 是一个面向数学与编程爱好者的在线题库，题目涵盖数论、组合数学、算法优化等多个领域。本项目旨在：

- 📝 提供清晰的代码实现
- 🧮 深入剖析背后的数学原理
- 💡 探索高效的算法策略

## 技术栈

- **语言**: Rust
- **构建工具**: Cargo

## 项目结构

```
project-euler/
├── 1/
│   ├── 1.rs          # 问题 1 的 Rust 实现
│   └── 1.markdown    # 问题 1 的数学思路与解析
├── Cargo.toml        # Rust 项目配置
└── README.md         # 项目说明文档
```

## 运行方式

每个问题都被配置为独立的二进制程序，可通过以下命令运行：

```bash
# 运行问题 1
cargo run --bin problem1

# 运行其他问题（以问题 N 为例）
cargo run --bin problemN
```

## 题目列表

| 编号 | 标题 | 难度 | 状态 |
|------|------|------|------|
| 1 | Multiples of 3 or 5 | ⭐ | ✅ |

## 学习笔记

每道题目的 `.markdown` 文件包含：

- 问题描述
- 数学建模与推导
- 算法复杂度分析
- 优化思路与技巧

## 贡献指南

欢迎提交 Issue 或 Pull Request 来：

- 指出代码或数学推导中的错误
- 分享更优雅的解法
- 补充新的题目解答

## 许可证

本项目采用 MIT 许可证开源。

## 相关链接

- [Project Euler 官网](https://projecteuler.net/)
- [Rust 官方文档](https://doc.rust-lang.org/)

---

*Keep solving, keep learning.* 🚀
