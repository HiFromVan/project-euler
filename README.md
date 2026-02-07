# Project Euler 刷题记录

我的 [Project Euler](https://projecteuler.net/) 解题笔记，用 Rust 写代码，顺便整理一下数学思路。

## 为什么做这个

Project Euler 的题目挺有意思的，既能练编程又能复习数学。每次做完题都想记录一下思路，不然过段时间就忘了当时是怎么想的。

## 怎么跑代码

```bash
# 跑第 1 题
cargo run --bin problem1

# 跑第 N 题
cargo run --bin problemN
```

## 目录结构

```
project-euler/
├── 1/
│   ├── 1.rs          # 代码实现
│   └── 1.markdown    # 解题思路和数学推导
├── 2/
│   ├── 2.rs
│   └── 2.markdown
└── ...
```

每道题一个文件夹，代码和笔记放一起，方便查看。

## 已完成题目

| # | 题目 | 难度 | 笔记 |
|---|------|------|------|
| 1 | Multiples of 3 or 5 | ⭐ | [思路](1/1.markdown) |

## 关于笔记

每道题的 `.markdown` 文件里会写：
- 题目理解
- 数学推导（如果有的话）
- 代码思路
- 踩过的坑

## 技术栈

- Rust（练手 + 性能好）
- Cargo（构建工具）

---

慢慢刷，不着急 🐢
