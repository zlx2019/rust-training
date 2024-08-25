# Rust Tools
## Rustup
[`rustup`](https://rust-lang.github.io/rustup/) 是 Rust 的安装程序，也是它的依赖管理工具，通过 `rustup`可以很方便的管理多版本Rust。

在 Unix 系统下安装:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Rust开发环境中，所有的工具都会安装到`~/.cargo/bin`目录下。

安装是否成功，可通过如下命令验证：
```shell
$ rustup -V
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.77.2 (25ef9e3d8 2024-04-09)`

$ cargo -V
cargo 1.77.2 (e52e36006 2024-03-26)

$ rustc --version
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

## Rustup Commands

### show

查看正在使用的和已安装的工具链信息；

```bash
$ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/zero/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
1.73.0-aarch64-apple-darwin
1.77.0-aarch64-apple-darwin
1.77.2-aarch64-apple-darwin (default)
1.63-aarch64-apple-darwin
1.65-aarch64-apple-darwin

active toolchain
----------------

1.77.2-aarch64-apple-darwin (default)
rustc 1.77.2 (25ef9e3d8 2024-04-09)
```

### update

更新Rust和Rustup工具链：

```bash
$ rustup update
```

### check

检查 Rust 工具链和 rustup 的更新：

```bash
$ rustup check
stable-aarch64-apple-darwin - Up to date : 1.80.1 (3f5fd8dd4 2024-08-06)
1.63-aarch64-apple-darwin - Up to date : 1.63.0 (4b91a6ea7 2022-08-08)
1.65-aarch64-apple-darwin - Up to date : 1.65.0 (897e37553 2022-11-02)
rustup - Up to date : 1.27.1
```

### default

设置默认工具链：

```bash
# 查看已安装的工具链
$ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/zero/.rustup

installed toolchains
--------------------

stable-aarch64-apple-darwin
1.73.0-aarch64-apple-darwin (default)
1.77.0-aarch64-apple-darwin
1.77.2-aarch64-apple-darwin
1.63-aarch64-apple-darwin
1.65-aarch64-apple-darwin

active toolchain
----------------

1.73.0-aarch64-apple-darwin (default)
rustc 1.73.0 (cc66ad468 2023-10-03)


# 设置Rust版本为当前最新稳定版本
$ rustup default stable
info: using existing install for 'stable-aarch64-apple-darwin'
info: default toolchain set to 'stable-aarch64-apple-darwin'

  stable-aarch64-apple-darwin unchanged - rustc 1.80.1 (3f5fd8dd4 2024-08-06)

# 设置Rust工具链为指定版本
$ rustup default 1.73.0
info: using existing install for '1.73.0-aarch64-apple-darwin'
info: default toolchain set to '1.73.0-aarch64-apple-darwin'

  1.73.0-aarch64-apple-darwin unchanged - rustc 1.73.0 (cc66ad468 2023-10-03)
```

### toolchain

Rust工具链相关命令。

**查看已安装的工具链**

```bash
$ rustup toolchain list
stable-aarch64-apple-darwin
1.73.0-aarch64-apple-darwin (default)
1.77.0-aarch64-apple-darwin
1.77.2-aarch64-apple-darwin
1.63-aarch64-apple-darwin
1.65-aarch64-apple-darwin
```

**安装 && 卸载工具链**

```bash
$ rustup toolchain install nightly
...

$ rustup toolchain list
stable-aarch64-apple-darwin
nightly-aarch64-apple-darwin
1.73.0-aarch64-apple-darwin (default)
1.77.0-aarch64-apple-darwin
1.77.2-aarch64-apple-darwin
1.63-aarch64-apple-darwin
1.65-aarch64-apple-darwin

$ rustup toolchain uninstall nightly
```

### target

用于交叉编译平台的管理。

**list**

查看可用的和已安装的交叉编译目标平台

```bash
$ rustup target list
aarch64-apple-darwin (installed)
...
x86_64-apple-darwin
x86_64-pc-windows-gnu
x86_64-pc-windows-msvc
...
```

**add**

安装所需要的交叉编译平台，以`x86-64`架构，Linux平台为例：

```bash
$ rustup target add x86_64-unknown-linux-gnu
info: downloading component 'rust-std' for 'x86_64-unknown-linux-gnu'
info: installing component 'rust-std' for 'x86_64-unknown-linux-gnu'

$ rustup target list
...
x86_64-unknown-linux-gnu (installed)
x86_64-unknown-linux-musl
...
```

然后交叉编译时指定该平台即可：

```bash
$ rustup build --target x86_64-unknown-linux-gnu
```





### component

修改工具链已安装的组件。



### run

`rustup run` 命令的作用是**在执行特定命令时临时指定使用某个特定的 Rust 工具链**，而无需更改全局或项目的默认工具链设置。

假设我现在同时安装了多个Rust工具链，如`stable`、`beta`、`nightly`等版本，但是我想使用其中某一个特定版本来运行或打包一个工程时，就可以使用`rustup run`来实现，而不用更改全局默认的工具链版本。举例如下：

1. 创建一个最基本的工程

   ```bash
   cargo new hello
        Created binary (application) `hello` package
   ```

2. 使用`nightly`来运行这个工程

   ```bash
   $ rustup run nightly cargo run
      Compiling hello v0.1.0 (/Users/zero/Woker/hello)
       Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
        Running `target/debug/hello`
   Hello, world!
   ```

### help


```shell
rustup 1.27.1 (54dd3d00f 2024-04-24)

The Rust toolchain installer

Usage: rustup [OPTIONS] [+toolchain] [COMMAND]

Commands:
  show         Show the active and installed toolchains or profiles
  update       Update Rust toolchains and rustup
  check        Check for updates to Rust toolchains and rustup
  default      Set the default toolchain
  toolchain    Modify or query the installed toolchains
  target       Modify a toolchain's supported targets
  component    Modify a toolchain's installed components
  override     Modify toolchain overrides for directories
  run          Run a command with an environment configured for a given toolchain
  which        Display which binary will be run for a given command
  doc          Open the documentation for the current toolchain
  man          View the man page for a given command
  self         Modify the rustup installation
  set          Alter rustup settings
  completions  Generate tab-completion scripts for your shell
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [+toolchain]  release channel (e.g. +stable) or custom toolchain to set override

Options:
  -v, --verbose  Enable verbose output
  -q, --quiet    Disable progress output
  -h, --help     Print help
  -V, --version  Print version

Discussion:
    Rustup installs The Rust Programming Language from the official
    release channels, enabling you to easily switch between stable,
    beta, and nightly compilers and keep them updated. It makes
    cross-compiling simpler with binary builds of the standard library
    for common platforms.

    If you are new to Rust consider running `rustup doc --book` to
    learn Rust.
```



## Cargo