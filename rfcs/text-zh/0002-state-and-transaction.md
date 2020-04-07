- title: 状态与事务

## SUMMARY

描述了Lightcore中对状态 `State` 概念的表达方式，和对 `State` 进行修改的 `Transaction` 事务的结构。

## Motivation

提供一个更加高效的，具有兼容性和扩展性的 `State` 和`Transaction` 结构。

## Detailed design

### 状态

Lightcore本质上是一个多节点状态管理框架，本质上Lightcore中管理的状态就是一个具有一定长度的二进制数组，我们可以使用这样的代码表达状态：

```rust
pub struct State {
    pub capacity: usize,
    pub state: Vec<u8>,
    pub lock: Option<Script>,
}
```

这样的结构是Lightcore框架中状态的基础表达形式，Lightcore的核心功能就是保证系统中的状态集合按照区块为时间周期，在Lightcore的各个节点上进行复制（replicate）。

### 状态锁定

状态中的lock是一个锁定脚本，外部对状态进行修改时，需要传递一个输入参数作为解锁参数。lock脚本将输入参数作为脚本的输入。脚本接收到输入数据之后进行计算，只有最终输出结果为`true`才可以对状态进行修改。

Lightcore中的事务结构利用输入来选定一个需要修改的状态，并传递输入参数，进行解锁。

```rust
pub struct Argument {
    pub length: usize,
    pub data: Vec<u8>
}

pub struct Input {
    pub txid: Txid,
    pub n: u64,
    pub argument: Argument,
}
```

### 状态修改

Lightcore中提供了三种方式对系统中的状态进行修改。

#### 直接修改

外部系统可以直接指定目标状态变化后的状态，

### 交易基础结构

Lightcore将UTXO结构进行扩展，利用UTXO表达状态的转换。