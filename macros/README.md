# 什么是元编程？

元编程（MetaProgramming）是一种广义的概念，指的是**编写能够操作、生成或修改代码的程序**。

# 什么是宏？

宏（Macro）是一种能够通过**代码生成代码**的技术。并且宏是由编译器在**编译阶段**完成的操作，本质上来说宏也是**元编程**的一种体现形式。

# 前置补充

> 在真正开始学习 Rust宏 之前，先来铺垫一些相关知识与概念。

## Rust 编译

### **标识化（Tokenization）**

Rust程序编译过程的第一个阶段是[标记解析](https://en.wikipedia.org/wiki/Lexical_analysis#Tokenization)。在此阶段中，源代码将被转换成一系列的标记（token）。

> token：无法被分割的词法单元；在编程语言世界中等价于“单词”。

Rust包含多种标记，比如：

- 标识符 (identifiers): `foo`, `Bambous`, `self`, `we_can_dance`, `LaCaravane`, …
- 字面值 (literals): `42`, `72u32`, `0_______0`, `1.0e-40`, `"ferris was here"`, …
- 关键字 (keywords): `_`, `fn`, `self`, `match`, `yield`, `macro`, …
- 符号 (symbols): `[`, `:`, `::`, `?`, `~`, `@`[1](https://zjp-cn.github.io/tlborm/syntax-extensions/source-analysis.html#wither-at), …

需要注意的是：

1. `self`既是一个标识符又是一个关键词。
2. 关键词里列有一些可疑的家伙，比如 `yield` 和 `macro`。 它们在当前的Rust语言中并没有任何含义，但编译器的确会把它们视作关键词进行解析。 这些词语被保留作语言未来扩充时使用。

> 某些语言的宏系统就工作于此阶段，比如C/C++的宏就是在本阶段进行处理，但是Rust并非如此。

### 语法解析（Parsing）





# Rust 宏

在Rust中，宏分为两大类：**声明式宏**、**过程宏**。而过程宏又分为三种：**派生宏**、**属性宏** 和 **函数宏**。

- **声明式宏**
- **过程宏**
  - **派生宏**
  - **属性宏**
  - **函数宏**

## 与函数的区别

- 编译期处理：宏在编译时展开，没有运行时开销。
- 可变参数处理：宏可以接受任意数量的参数，最终都会在编译时展开。
- 代码生成：减少大量的重复代码，并且扩展性强。

<hr>



## 宏的展开

```rust
fn main() {
    println!("Hello, {}, age is {}", "Zero", 18);
}
```

这是一个简单的声明式宏，可以通过`cargo expand` 命令来查看宏展开后的源码：

```bash
$ cargo expand -p macros  

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, {0}, age is {1}\n", "Zero", 18));
    };
}
```

<hr>


# 声明式宏

```c
#define MUL(a, b) a * b
```

熟悉C/C++的人应该对这句代码不陌生，Rust的声明式宏其实也是一样的功能，本质上也是一种代码模板替换，只不过是在此基础上添加了**模式匹配**的功能，借此使得功能更加强大。

**声明式宏(declarative macros)** 是Rust中使用最广泛的宏，也称为"Macros by Example"。或者你不知道但你一定用过，例如下列这些都属于声明式宏：

```rust
// 向终端输出一句文本
println!("Hello {}", "World");

// 创建一个Vec
let v1: Vec<i32> = vec![1,2,3];

// 格式化字符串
let s = format!("Hello, {}", "World");
```

声明式宏本质上来说就是一种**模板替换**，它允许写出类似于 `match` 的代码：接收一个表达式，然后根据表达式的结果与多个模式进行匹配，一旦匹配匹配了某个模式，则执行该模式关联的代码。

```rust
match target {
  模式1 => 表达式1,
  模式2 => {
    语句1;
    语句2;
    表达式3
  },
  _ => 表达式1
}
```

语法结构如下：

```rust
MacroRulesDefinition :
   macro_rules ! IDENTIFIER MacroRulesDef

MacroRulesDef :
      ( MacroRules ) ;
   | [ MacroRules ] ;
   | { MacroRules }

MacroRules :
   MacroRule ( ; MacroRule )* ;?

MacroRule :
   MacroMatcher => MacroTranscriber

MacroMatcher :
      ( MacroMatch* )
   | [ MacroMatch* ]
   | { MacroMatch* }

MacroMatch :
      Tokenexcept $ and delimiters
   | MacroMatcher
   | $ ( IDENTIFIER_OR_KEYWORD except crate | RAW_IDENTIFIER | _ ) : MacroFragSpec
   | $ ( MacroMatch+ ) MacroRepSep? MacroRepOp

MacroFragSpec :
      block | expr | expr_2021 | ident | item | lifetime | literal
   | meta | pat | pat_param | path | stmt | tt | ty | vis

MacroRepSep :
   Tokenexcept delimiters and MacroRepOp

MacroRepOp :
   * | + | ?

MacroTranscriber :
   DelimTokenTree
```

核心组成：

- `MacroRulesDef`：宏界定符，表示宏的开始和结束。可以为`()`、`[]`、`{}`三种。
- `MacroRules`：宏的所有匹配规则，包含一个或多个`MacroRule`。
- `MacroRule`：匹配规则，包含一个`MacroMatcher`和对应的`MacroTranscriber`。
- `MacroTranscriber`：宏最终展开后的代码，可以理解为转录器。
- `MacroFragSpec`：捕获片段分类符，表示要捕获的内容类型。
- `MacroRepSep`：重复分隔符，一般用于可变长参数的宏，如`vec![1,2,3]```
- `MacroRepOp`：重复操作符，可以是`*`、`+` 和 `?`

示例：

```rust
macor_rules! macor_name {
  // 可以拥有一条或多条规则
	pattern1 => expression1;
  pattern2 => {
 		statement1;
    statement2;
    expression_3
  };
  pattern3($name: expr) => expression1;
  //...
}
```

## 元变量

matcher 还可以包含捕获 (captures)。即基于某种通用语法类别来匹配输入，并将结果捕获到元变量 (metavariable) 中，然后将替换元变量到输出。

捕获的书写方式是：先写美元符号 `$`，然后跟一个**标识符(Identifiers)**，然后是冒号 `:`，最后是**捕获方式**，比如：

```rust
// 表示会匹配一个 `idnet` 标识符，然后绑定到名为`name`的宏变量上。
($name: ident) => expression;
```



<hr>

## 片段分类符

捕获方式又被称为**片段分类符(fragment-specifier)**，是用来表示调用宏时传入的参数类型。必须是以下的一种：

- [`block`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#block)：一个块（比如一块语句或者由大括号包围的一个表达式）
- [`expr`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#expr)：一个表达式 (expression)
- [`ident`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#ident)：一个标识符 (identifier)，包括关键字 (keywords)
- [`item`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#item)：一个条目（比如函数、结构体、模块、`impl` 块）
- [`lifetime`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#lifetime)：一个生命周期注解（比如 `'foo`、`'static`）
- [`literal`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#literal)：一个字面值（比如 `"Hello World!"`、`3.14`、`'🦀'`）
- [`meta`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#meta)：一个元信息（比如 `#[...]` 和 `#![...]` 属性内部的东西）
- [`pat`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#pat)：一个模式 (pattern)
- [`path`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#path)：一条路径（比如 `foo`、`::std::mem::replace`、`transmute::<_, int>`）
- [`stmt`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#stmt)：一条语句 (statement)
- [`tt`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#tt)：单棵标记树
- [`ty`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#ty)：一个类型
- [`vis`](https://zjp-cn.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html#vis)：一个可能为空的可见性修饰符（比如 `pub`、`pub(in crate)`）

### block

`block`分类符表示一段代码块。由`{`开始，接着是一些语句，最后是可选的表达式，然后以`}`结束。

```rust
```





## 重复捕获

这些操作符允许宏匹配重复出现的语法结构：

- `*`：匹配0次或多次
- `+`：匹配1次或多次
- `?`：匹配0次或1次

<hr>

## 练习案例

1. 编写一个我们自己的`vec!`宏。

   ```rust
   #[macro_export]
   macro_rules! vector {
       // 当可变长参数为空时，则直接返回一个空的 Vec
       () => {Vec::new()};
   
       // MacroMatcher 格式:  $ ( MacroMatch+ ) MacroRepSep? MacroRepOp
       // `$()` 表示括号内的模式可以重复多次
       // `$item: expr` 表示捕获片段的元变量名和捕获片段的类型为 一个表达式
       // ',' 表示重复匹配表达式之间的分隔符
       // '+' 表示可以重复1次或多次.
       ( $( $item: expr ),+ ) => {
           {
               let mut vec = Vec::new();
               $(
                   vec.push($item);
               )*
               vec
           }
       }
   }
   ```

   使用它

   ```rust
   fn main() {
       let vec1 =  vector![1,2,3];
   }
   ```

   然后查看展开后的源码：

   ```rust
   fn main() {
       let vec1 = {
           let mut vec = Vec::new();
           vec.push(1);
           vec.push(2);
           vec.push(3);
           vec
       };
   }
   ```

   这样一个简单的宏就写好了，但是还需要进一步优化一下它：

   ```rust
   #[macro_export]
   macro_rules! vector {
       // vector![] => []
       () => {Vec::new()};
     
       // vector![3, 4]; => [3,3,3,3]
       ($elem: expr; $size: expr) => (Vec::from_elem($elem, $size));
   
     	// vector![1,2,3]; => [1, 2, 3]
       ( $( $item: expr ),+ $(,)?) => {
           {
               <[_]>::into_vec(Box::new([$($item),+]))
           }
       }
   }
   
   ```

   这样就和标准库的`vec!`差不多啦!

**代码生成**

我们也可以通过声明式宏来定义方法和结构体，如下:

```rust
macro_rules! define_fn {
    // name: 方法名 --> 标识符
    // rt:   返回值类型 --> 类型
    // body: 方法体 --> 代码块
    ($name: ident, $rt: tt, $body: block) => {
        fn $name () -> $rt {
            $body
        }
    };
}

/// 定义结构体
macro_rules! define_struct {
    // 结构体名称
    // 结构体属性列表 field : type
    (
        $name: ident,
        $( $field_name:ident : $field_type:ty ),*
    ) => {
        #[derive(Debug)]
        struct $name {
            // 遍历属性类型列表
            $( $field_name: $field_type),*
        }
        impl $name {
            fn new ($($field_name: $field_type),*) -> Self {
                Self{
                    $($field_name),*
                }
            }
        }
    };
}


fn main() {
    define_fn!(sum, i32, {
        let x = 10;
        let y = 10;
        x * y
    });
    let res=  sum();
    println!("x * y = {}", res);


    define_struct!(User, name: String, age: u32, address: String);
    let user = User{name: String::from("Zero"), age: 18, address: "深圳".to_string()};
    println!("{:?}", user)
}
```









# 过程宏



## 派生宏

**派生宏(Derive macros)** 是 Rust 过程宏的其中一种，它可以实现自动为类型**派生**出特定的特征（trait）代码。比如我们平常经常用到的 `#[derive(...)]` 就是派生宏。

如下示例：

```rust
#[derive(Debug, Default)]
struct User;
```

该示例通过`#[derive]`宏为`User`结构体自动实现了`Debug`和`Default`两个trait，该示例展开后的源码为如下：

```rust
struct User;
#[automatically_derived]
impl ::core::fmt::Debug for User {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "User")
    }
}
#[automatically_derived]
impl ::core::default::Default for User {
    #[inline]
    fn default() -> User {
        User {}
    }
}
```

所以说，`#[derive]`派生宏的作用主要就是为一些类型自动实现一些特征。



<hr>

## 属性宏







<hr>

## 函数宏





