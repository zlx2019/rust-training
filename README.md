# Rust Training.

<img src="https://music9501.oss-cn-beijing.aliyuncs.com/%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80Logo/Rust.png" style="zoom:50%;" />

通过陈天老师的Rust训练营，进一步对Rust进行深入学习.

# 前置配置

## 安装Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## VsCode配置

需要安装的插件列表

- **crates**：Rust的依赖管理插件
- **Even Better TOML**： toml配置文件支持
- **Error Lens**：错误提示插件
- **indent-rainbow**：缩减显示优化插件
- **Prettier-Code formatter**：代码格式化插件
- **rust-analyzer**：Rust编程语言插件
- **Rust Test lens**：Rust测试支持
- **Rust Test Explorer**： Rust测试概览
- **TODO Highlight**：TODO高亮

## pre-commit

pre-commit是一个代码检查工具，可以在提交代码前进行检查。

```bash
pipx install pre-commit
```

安装成功后运行 `pre-commit install`即可.

## cargo-deny

cargo deny 是一个Cargo插件，可以用于检查依赖的安全性问题.

```bash
cargo install --locked cargo-deny
```

## cargo-nextest

cargo nextest 是一个Rust增强测试工具，比原生的Cargo自带的单元测试要效率更高.

```bas
cargo install cargo-nextest --locked
```

## Cargo-generate

cargo generate 是一个用于生成项目模板的工具。它可以使用已有的 github repo 作为模版生成新的项目。

```bash
cargo install cargo-generate
```

例如使用 `tyr-rust-bootcamp/template` 模版生成基本的代码：

```bash
cargo generate tyr-rust-bootcamp/template
```



# 基础补充

## 值在堆上还是栈上？

在开始学习Rust之前，我们先来回顾一下基础知识：

> 我们知道，**变量**(这里指局部变量)都是存放在**内存**中的，而内存又分为**堆**和**栈**，那么究竟是分配在**栈**上还是**堆**上？

这个问题对于学习Rust至关重要，不同的编程语言，对于内存的管理都有所不同，那么变量的分配与存储也会产生些许的差异，从而产生性能之间的差距。

- Java：**基本类型**存储在栈中，而**引用类型（对象）**存储在堆中。

- Go：通过**逃逸分析**优化，确定变量是放在堆中还是栈上，比如一个变量在函数结束后，依然被引用着，那么则会放在堆中，否则优先放在栈上。
- C/C++：由开发者自主选择。

其实Rust在这方面类似于 Go 和 C/C++的结合，在Rust中，**在编译期可以确定大小的的变量，是默认放在栈上**，当然也可以**显式的放在堆中**。



## 栈&栈帧

**栈(Stack)**本身是一种数据结构，遵循与**后进先出(LFOL)**的特性。而我们常说的**内存中的栈**，其实本质上就是**进程空间**中的**一片连续的内存段**，将这段内存以栈结构的特性进行读写。

**栈**是程序运行的基础，主要用于为函数分配内存空间。当一个函数被调用时，就会在栈中分配出一块连续的内存空间，这块内存被称之为**栈帧(Stack frame)**，然后将这个栈帧压入栈中(栈顶)。

我们知道，栈是自顶向下增长的，也就是**栈底**的**帧**通常是最先调用的函数(排除**入口帧**)，也就是**main()**函数对应的**帧**，而随着**main**函数中一层一层调用更多的函数，一个又一个新的**栈帧**会被压入到**栈顶**，调用链结束后，又会一层一层的弹出，把内存进行释放，直到**main**函数也被出栈，进程结束。

在调用的过程中，**一个新的帧会分配足够的空间存储寄存器的上下文**。在函数里使用到的通用寄存器会在栈保存一个副本，当这个函数调用结束，通过副本，可以恢复出原本的寄存器的上下文，就像什么都没有经历一样。此外，函数所需要使用到的局部变量，也都会在帧分配的时候被预留出来。

<img src="https://movies-bucket.oss-cn-beijing.aliyuncs.com/%E5%87%BD%E6%95%B0%E8%B0%83%E7%94%A8%E6%A0%88%E7%BB%93%E6%9E%84.png" style="zoom:75%;" />

那么一个函数运行时，怎么确定究竟需要一个多大内存的**栈帧**呢？

这就需要借助于**编译器**，在编译并优化代码阶段，一个函数就是一个最小的**编译单元**，在这个函数里，编译器得知道要用到哪些寄存器、栈上要放哪些局部变量，而这些都要在编译时确定。所以编译器就需要明确每个局部变量的大小，以便于预留空间。

所以说：**在编译时，一切无法确定大小或者大小可以动态改变的数据，都无法安全的放在栈上，最好放在堆中**。

比如说，下面有一个函数，参数是一个字符串：

```rust
fn foo(name: String){}

// call foo
foo("Jay".to_string());
```

**String(字符串)**的数据结构，在编译时是大小不确定的，运行时执行到具体的代码才知道大小，所以，我们无法把字符串本身放在栈上，只能先将其放在堆上，然后在栈上分配对应的指针，引用堆上的内存。

## 栈的问题

通过上面的讲述，我们可以直观的感受到，**栈上的内存分配与释放是非常高效的**，只需要改动 **SP(stack pointer 栈指针) **寄存器的值即可，比如说一个函数的栈帧需要 32Byte，那么只需要将 **SP寄存器**向下移动32个字节即可：

```assembly
sub sp, 32
```

这样就分配出了一个大小为 32Byte 的**栈帧**内存，当该函数结束后，只需要将 **SP寄存器**向上移动32个字节即可，这样就表示栈帧内存被释放：

```assembly
add sp, 32
```

栈上内存的分配与释放，只是动动寄存器，不涉及额外计算也不涉及系统调用等等，因此效率很高。所以理论上说，只要可能，我们应该把变量分配在栈上，这样可以达到更好的运行速度。

那么为什么我们在实际的工作中，有时候又要避免将大量的数据分配在栈上呢？

**这主要是考虑到调用栈的大小，避免栈溢出(stack overflow)**。一旦当前程序的调用栈超出了系统允许的最大栈空间，无法创建新的帧，来运行下一个要执行的函数，就会发生栈溢出，这时程序会被系统终止，产生崩溃信息。

## 堆

关于堆，暂且不谈论。

## 小结

今天我们重新回顾基础概念，分析了栈和堆的特点。

- 对于**存入栈上的值，它的大小在编译期就需要确定**。栈上存储的变量生命周期在当前调用栈的作用域内，无法跨调用栈引用。

- **堆可以存入大小未知或者动态伸缩的数据类型**。堆上存储的变量，其生命周期从分配后开始，一直到释放时才结束，因此堆上的变量允许在多个调用栈之间引用。但也导致堆变量的管理非常复杂，手工管理会引发很多内存安全性问题，而自动管理，无论是 GC 还是 ARC，都有性能损耗和其它问题。

一句话对比总结就是：**栈上存放的数据是静态的，固定大 小，固定生命周期；堆上存放的数据是动态的，不固定大小，不固定生命周期**。

# Rust核心概念





# 好用的依赖库

## anyhow

**anyhow**是Rust中非常流行的一个错误处理库，使用它可以更方便的进行错误处理.

```bash
cargo add anyhow
```

## serde

**serder**是Rust中流行度 Top One 级别的一个依赖库，它用于处理数据的**序列化**与**反序列化**.

```bash
cargo add serde --features derive
```

**serder_json**主要用于序列化 JSON 格式.

```bash
cargo add serde_json
```
