# catr

## 项目简介
`catr` 是一个用 Rust 编写的命令行工具，用于读取和显示文件内容。

## 功能特性
- 支持读取多个文件
- 支持从标准输入读取内容
- 错误处理：当文件不存在时，输出错误信息

## 安装说明
1. 克隆仓库：
    ```sh
    git clone ...
    cd catr
    ```
2. 构建项目：
    ```sh
    cargo build --release
    ```

## 使用示例
1. 读取文件内容：
    ```sh
    ./target/release/catr <file1> <file2> ...
    ```
2. 从标准输入读取内容：
    ```sh
    echo "Hello, world!" | ./target/release/catr -
    ```

## 测试说明
运行测试：
```sh
cargo test
```

## 贡献指南
欢迎贡献代码！请提交 Pull Request 或报告问题。

## 许可证信息
本项目使用 MIT 许可证。
