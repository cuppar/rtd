# Learn Rust by 500 lines code
[![crate_version](https://img.shields.io/crates/v/rtd-tutorial)](https://crates.io/crates/rtd-tutorial)
[![crate_downloads](https://img.shields.io/crates/d/rtd-tutorial?color=blue)](https://crates.io/crates/rtd-tutorial)
[![license](https://img.shields.io/crates/l/rtd-tutorial?color=red)](https://github.com/cuppar/rtd/blob/master/LICENSE)

English | [中文](https://github.com/cuppar/rtd/blob/master/readme_zh.md#500%E8%A1%8C%E4%BB%A3%E7%A0%81%E5%AD%A6%E4%BC%9Arust)

- RTD (Rust To Do) is a todo app cli tool write by 500 lines Rust code. (exclude space lines/comments/long line break display/test code)
- RTD is also a [tutorial](https://github.com/cuppar/rtd/wiki), designed to learn Rust by doing.

![rtd_list_all](Tutorial/doc/img/readme/rtd_list_all.png)

## Table of contents

- [What can you learn from rtd?](#what-can-you-learn-from-rtd)
- [Prepare](#prepare)
- [Install](#install)
- [Usage](#usage)
- [About author](#about-author)

---

## What can you learn from RTD?

#### Assuming you know nothing about Rust, through the [Learn Rust by 500 lines code](https://github.com/cuppar/rtd/wiki) tutorial, step by step to build the project from scratch, you will learn:
  - Rust common syntax
  - Rust module system (`package`/`binary crate`/`library crate`/`mod`/`pub`/`use`)
  - Rust ownership model (Bernstein conditions)
  - Rust lifetime
  - Rust error/null handling model (`Result<T, E>`/`Option<T>`)
  - Rust generic
  - Rust pattern matching (`if let` , `match` ...)
  - Rust unit test
  - Rust file I/O (`File Seek`/`Buffed File I/O`)
  - Rust manipulating environment variables
  - Rust command line parameter parsing
  - Rust release package to crate.io
  - Layered abstraction (data storage layer/model mapping layer/data model layer/business logic layer/application interface layer/user interface layer)
  - Handwritten serialization/deserialization from scratch
  - Implement the recycle bin function (virtualization concept)

###### Architecture

![rtd_arch](Tutorial/doc/img/readme/rtd_arch.svg)

###### Storage
Use a local file `$HOME/.rtd.csv` store all data

![data_storage](Tutorial/doc/img/readme/csv.png)
![rtd_list_all](Tutorial/doc/img/readme/rtd_list_all.png)

#### After completing this tutorial or installing RTD directly, you will get:
  - Extremely lightweight and concise todo command line application
  - Cross-platform, Rust's excellent build system naturally supports cross-platform
  - Safe, supports recycle bin, completely local, no network, no database
  - All data storage uses only one local csv file, which can be switched between different machines by copying the csv file

---
If you like my tutorial, don't forget to give me a star~
---

## Prepare

- Rust is naturally cross-platform. This project is built and tested in the `linux` environment, and `Windows`/`MacOS` are also supported. You can choose to download the corresponding `Cargo` according to your own operating system.
- `Cargo` Rust's package management and build tool, can be installed directly through the [Rust official website](https://www.rust-lang.org/tools/install) `rustup` one line command. Then, all things will be done by `Cargo`, so cute, right?

## Install

#### Via `crate.io` :

```bash
cargo install rtd-tutorial
```

#### Or via `git repo` :
```bash
git clone https://github.com/cuppar/rtd.git
cargo install --path rtd
```

## Usage

#### View help document
```bash
rtd -h
rtd --help
```

![rtd_help_summary](Tutorial/doc/img/readme/rtd_help_summary.png)
![rtd_help](Tutorial/doc/img/readme/rtd_help.png)

#### Add a todo
```bash
rtd -a <item-name>
rtd --add <item-name>
```

![rtd_add](Tutorial/doc/img/readme/rtd_add.png)

#### List all uncompleted todos
```bash
rtd
rtd -l
rtd -l uncompleted
rtd --list
rtd --list uncompleted
```

![rtd_list_uncompleted](Tutorial/doc/img/readme/rtd_list_uncompleted.png)

#### Complete a todo
```bash
rtd -c <item-id>
rtd --complete <item-id>
```

![rtd_complete_item](Tutorial/doc/img/readme/rtd_complete_item.png)

#### List all completed todos
```bash
rtd -l completed
rtd --list completed
```

![rtd_list_completed](Tutorial/doc/img/readme/rtd_list_completed.png)

#### Uncomplete a todo
```bash
rtd -u <item-id>
rtd --uncomplete <item-id>
```

![rtd_uncomplete_item](Tutorial/doc/img/readme/rtd_uncomplete_item.png)

#### Throw a todo into the recycle bin
```bash
rtd -d <item-id>
rtd --delete <item-id>
```

![rtd_delete_item](Tutorial/doc/img/readme/rtd_delete_item.png)

#### List all recycle bin todos
```bash
rtd -l deleted
rtd --list deleted
```

![rtd_list_deleted](Tutorial/doc/img/readme/rtd_list_deleted.png)

#### Restore a todo from the recycle bin
```bash
rtd -r <item-id>
rtd --restore <item-id>
```

![rtd_restore_item](Tutorial/doc/img/readme/rtd_restore_item.png)

#### Physically destroy a todo
```bash
rtd --destroy <item-id>
```

![rtd_destroy_item](Tutorial/doc/img/readme/rtd_destroy_item.png)

#### Empty recycle bin
```bash
rtd --destroy-deleted
```

![rtd_destroy_deleted](Tutorial/doc/img/readme/rtd_destroy_deleted.png)

#### List all todos
```bash
rtd -l all
rtd --list all
```

![rtd_list_all](Tutorial/doc/img/readme/rtd_list_all.png)

#### Clear all todos
```bash
rtd --clear
```

![rtd_clear](Tutorial/doc/img/readme/rtd_clear.png)

## About author

Cuppar He(He Zhiying), software development engineer, likes programming, technical writing, learning new things, especially computer science, worked for [SAP](https://www.sap.com/)([World Top 100](https://www.rankingthebrands.com/Brand-detail.aspx?brandID=22)) and [Alibaba Group](https://www.alibabagroup.com/)([World Top 100](https://www.rankingthebrands.com/Brand-detail.aspx?brandID=6245) & Chinese internet giant). I am currently in Gap Year, if you are looking for a software development engineer and can provide a high-quality offer(Both remote and on-site), please contact me `cuppar.hzy@gmail.com`.