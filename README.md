# 1. WHY Rust
性能、安全、协作

## 1.1 性能
零成本抽象是 Rust 设计哲学中一个非常重要的指导规范，在 Rust 本身设计和社区里都会奉行这一原则，我们可以简单理解成两个要点：
1. 没有用到的功能，不需要编译和运行时开销，不用给不需要的东西付费；
2. 那如果是需要用的功能，你也不可能优化的更好，在最终机器码层面上，Rust 可以帮你做到最优；


## 1.2 安全
Rust 是如何做到内存安全的呢? 
它有三个重要的概念，让我们一一来看一下：
1. Ownership 所有权
2. Borrowing 借用
3. Liftime 生命周期

### 1.2.1 所有权：
```rust
fn main() {
    let x =String::from("hi") // owner: x   resource: "hi"   
    println!("{}",x)
} // free
```
变量 x 拥有值的所有权，也就是这里的 hi，最后在变量 x 出了作用域之后，hi 相关的内存就会被释放了。
Rust 里内存的管理是自动的，不需要垃圾回收参与，申请和释放的时机都是在编译时就决定了。

对于所有权，有三条规则：
- Rust 中的每一个值都有一个被称为其所有者（owner）的变量
- 值在任意时刻有且只有一个所有者：这可以避免像 多次释放 和 错误释放 等内存安全问题
- 当所有者（变量）离开作用域的时候，这个值将被丢弃（回收）：出作用域释放可以避免 memory leak 的问题

举个例子,下面这个程序会报错，因为它违背了所有权规则2：所有权是唯一的。let y = x实际上会把所有权从 x 转移到了 y，这个时候如果我们再去使用 x，Rust 就会报错

```rust
fn main() {
    let x = String::from("hi") 
    let y = x
    println!("{}",x)
} 
// error: borrow of moved value:`x`
```

### 1.2.2 借用：
上个例子中，讲了唯一所有权带来的内存安全保障，但是这不利于我们对值复杂的访问需求，这里我们介绍另外一个 Rust 概念——借用。

借用只有一条规则：
- 同时只能存在一个可变借用或者多个不可变借用

不可变借用
可以创建一个不可变借用来避免所有权转移，let y = &x，这样我们就能够在不打破单一所有权的同时，满足多个入口访问相同值的需求；换言之，借用只是共享了访问权限，不拥有值，它是只读的。

```rust
fn main() {
    let x = String::from("Hi");
    let y = & x; // immutable borrow
    println!("{}", x);
    println!("{}", y);
}
```

可变借用
有时候不仅需要共享只读权限，还需要去改变这个共享的值，这时候我们就需要一个可变的借用了let y = &mut x 

case 1

```rust
fn main() {
    let mut x = String::from("Hi"); // 注意这里x也需要申明为mut可变的，否则会报错
    let y = &mut x;
    y.push_str(" rust");
    println!("{y}"); 
}
```
case2 
```rust
fn main() {
    let mut x = String::from("Hi");
    let y = &mut x;
    y.push_str(" rust");
    let z = &x;
    println!("{z}");
    println!("{y}");
}
// error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
```
case2会报错，因为它违反了借用的规则：同时只能存在一个可变借用或者多个不可变借用。Rust不允许可变引用和不可变引用同时存在。


### 1.2.3 生命周期：
原理：一个资源能借用的时间，不能超过这个资源存在的时间（能借多久 Owner 说了算）

### 1.2.4 并发安全：


# 2. install Rust
## 2.1 install
```bash
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

mkdir -p ~/.cargo
// 将以下几行放到你的~/.cargo/config文件中
[source.crates-io]
replace-with = 'rsproxy'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true 

// 然后，执行以下命令：
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh

source "$HOME/.cargo/env"
```

## 2.2 cargo
Rust 社区使用 Cargo 来作为包管理和构建工具
我们先输入以下命令，来创建我们的项目目录：
cargo new project_name
```bash
project_name
├── Cargo.lock # Cargo 依赖版本锁定文件
├── Cargo.toml # Cargo 主要设置文件
└── src
    └── main.rs # Rust 程序入口
```
在目录下输入 cargo run 即可运行