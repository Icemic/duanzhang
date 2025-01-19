# 断章

一个字体信息查询和子集化工具。

## 特性

1. 支持 TrueType 字体和 OpenType 字体
2. 支持查询字体信息（名称、包含字符类型等）
3. 根据指定的范围创建字体子集

## 安装

### 从 Cargo 安装

```bash
cargo install --git https://github.com/Icemic/duanzhang.git
```

### 从源码安装

```bash
git clone
cd duanzhang
cargo install --path .
```

### 从预编译二进制文件安装

从 [Releases](./releases) 页面下载预编译的二进制文件。

## 使用

### 查询字体信息

```bash
duanzhang info /path/to/font.ttf
```

### 创建字体子集

```bash
duanzhang subset /path/to/font.ttf --output /path/to/output.ttf --charset ./charset.txt --presets kana

# 查看详细帮助
duanzhang subset --help
```

## License

本项目使用 Apache-2.0 OR MIT 许可证。
