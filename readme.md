# RTD (Rust ToDo)
English | [中文](https://github.com/cuppar/rtd/blob/master/readme_zh.md)

RTD是一个500行（不包括空行/注释/长行折断显示）Rust代码的命令行todo应用。

（目录）

## 你能从中得到什么？

- 如果你想学习Rust, 通过[500行Rust代码轻松构建Todo App(TODO)](https://github.com/cuppar/rtd)教程，一步步从零构建该项目，你将学会Rust的：
  - 常用语法
    - 流程结构
    - 函数
    - 闭包
    - 结构体
    - 枚举
    - 实现块
    - 特性
    - 属性
  - 模式匹配
    - `if let`
    - `match`
    - 解构赋值
  - 分层抽象
    - 数据模型层
    - 物理存储层
    - 业务逻辑层
    - 应用接口层
    - 用户界面层
  - 模块组织
    - `package`
    - `crate`
    - `module`
    - `pub`
    - `use`
    - `super`
    - `self`
  - 错误/空值处理
    - `Result`
    - `Option`
  - 所有权
    - 伯恩斯坦条件
    - 指针/悬空指针
    - 引用/悬空引用
    - 内存泄漏
    - 可变性
    - 借用
    - 复制语义
    - 移动语义
  - 单元测试
  - 文件读写
    - 文件/路径操作
    - 有缓冲的文件读写
    - `File Seek` 操作
  - 环境变量
    - 操作环境变量
  - 命令行
    - `clap` 极其方便的命令行库
  - 序列化反序列化
    - 手写csv和Rust对象之间的映射
  - 实现回收站

- 如果你想在命令行使用方便的todo应用，那么RTD是:
  - 极其轻量级且简洁的todo命令行应用
  - 跨平台，Rust项目优秀的构建系统天然支持跨平台
  - 安全，支持回收站，完全本地，不联网，无数据库
  - 所有数据存储仅使用一个本地csv文件，可以通过复制csv文件来在不同机器间切换

## 准备

- Rust 天然跨平台，本项目在 `linux` 环境构建测试，`Windows`/`MacOS` 同样支持，根据自身操作系统选择下载对应 `Cargo` 即可。
- `Cargo` Rust 包管理和构建工具, 可通过[官网](https://www.rust-lang.org/tools/install)`rustup`一行命令直接安装，接下来的事情，它会帮你全搞定，很可爱，不是吗？

## 安装

```bash
git clone https://github.com/cuppar/rtd.git
cd rtd
cargo install --path .
```

## 使用

#### 添加一个todo
```bash
rtd -a <item-name>
rtd --add <item-name>
```

![rtd_add](Tutorial/doc/img/rtd_add.png)

#### 列出所有未完成的todo
```bash
rtd
rtd -l
rtd -l uncompleted
rtd --list
rtd --list uncompleted
```

![rtd_list_uncompleted](Tutorial/doc/img/rtd_list_uncompleted.png)

#### 完成一个todo
```bash
rtd -c <item-id>
rtd --complete <item-id>
```

![rtd_complete_item](Tutorial/doc/img/rtd_complete_item.png)

#### 列出所有已完成的todo
```bash
rtd -l completed
rtd --list completed
```

![rtd_list_completed](Tutorial/doc/img/rtd_list_completed.png)

#### 标记一个todo为未完成
```bash
rtd -u <item-id>
rtd --uncomplete <item-id>
```

![rtd_uncomplete_item](Tutorial/doc/img/rtd_uncomplete_item.png)

#### 把一个todo扔进回收站
```bash
rtd -d <item-id>
rtd --delete <item-id>
```

![rtd_delete_item](Tutorial/doc/img/rtd_delete_item.png)

#### 列出所有回收站的todo
```bash
rtd -l deleted
rtd --list deleted
```

![rtd_list_deleted](Tutorial/doc/img/rtd_list_deleted.png)

#### 从回收站恢复一个todo
```bash
rtd -r <item-id>
rtd --restore <item-id>
```

![rtd_restore_item](Tutorial/doc/img/rtd_restore_item.png)

#### 物理销毁一个todo
```bash
rtd --destroy <item-id>
```

![rtd_destroy_item](Tutorial/doc/img/rtd_destroy_item.png)

#### 清空回收站
```bash
rtd --destroy-deleted
```

![rtd_destroy_deleted](Tutorial/doc/img/rtd_destroy_deleted.png)

#### 列出所有todo
```bash
rtd -l all
rtd --list all
```

![rtd_list_all](Tutorial/doc/img/rtd_list_all.png)

#### 清空所有todo
```bash
rtd --clear
```

![rtd_clear](Tutorial/doc/img/rtd_clear.png)

## 架构及存储

#### 架构图

![rtd_arch_zh](Tutorial/doc/img/rtd_arch_zh.svg)

#### 存储
使用一个本地文件 `$HOME/.rtd.csv` 存储所有数据

---
如果你喜欢我的教程，别忘了给我点个赞哦～
