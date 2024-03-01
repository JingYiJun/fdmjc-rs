# FDMJ Compiler in Rust

复旦大学 2024 春编译（H）实验，Rust 实现

- [x] 项目管理使用 [cargo-make](https://github.com/sagiegurari/cargo-make)
- [x] 使用 [pest](https://pest.rs) 库做词法分析和语法分析

## 环境配置

- Windows, Linux, MacOS 都行

- Rust: 最新的 stable 版本。安装和包镜像源参考 [rsproxy.cn](https://rsproxy.cn/)

- LLVM: 最新版本，16.0.0 以上，Windows 上可以用 msys2 安装

- cargo-make

```bash
cargo install --force cargo-make
```

## 运行方法

```bash
cargo make test-fdmj
```
