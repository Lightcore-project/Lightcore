# Lightcore

## 一、流程模块：

| 模块        | 功能                          |
|-------------|-------------------------------|
| JsonRPC     | 接收新 Txs                    |
| P2P         | Gossip 同步 Txs，连接共识区块 |
| 共识        | 同步区块                      |
| 交易池      | 存储未被打包交易              |
| Chain数据库 | 存储接收到的已被打包的交易    |


## 二、模块连接关系：
1. Jsonrpc <--> 交易池
2. 交易池 <--> 共识
3. 交易池 <--> P2P
4. P2P <--> 共识

## 三、模块具体流程与协议细节：

### 1. Jsonrpc线程

+ 方法名：SendRawTranscation
+ 接收参数：被签名的交易 Tx
+ fn accept_tx_bytes(Vec<u8>);
+ 具体操作：
  + 对交易进行签名验证。
  + 检查交易是否重复
  + 写入交易池
  + (随机选择k个节点发送，若曾经发送过这笔交易，则不再发送）P2P

### 2. 共识线程：
#### 2.1 定期读取固定数目的交易：（Loop）
+ 从交易池获取固定数目T交易
+ 对交易排序
+ 递交给Executor执行
+ 打包后进行共识

#### 2.2 共识线程收到区块：
+ 验证区块
+ 验证交易
+ 送入Executor执行
+ 存入Chain数据库
+ 触发事件

### 3. P2P线程：

#### 3.1 定期监听新信息：

+ 若是共识信息，传递给共识模块处理
+ 若是交易同步信息，传递给Jsonrpc，作为SendRawTranscation方法处理

## 四、数据传输验证
### 1. 交易数据
#### 1.1 交易结构：

| 字段名    | 类型              | 解释       |
|-----------|-------------------|------------|
| txid      | SHA3-256          | 交易 ID    |
| signature | Ed25519           | 交易签名   |
| to        | SHA3-256          | 交易操作   |
| from      | PublicKey         | 交易发起者 |
| nonce     | uint64            | 交易顺序   |
| operator  | SHA3-256          | 操作       |
| documents | Array of SHA3-256 | 文档列表   |
| data      | Bytes             | 额外数据   |


#### 1.2 交易签名流程
+ 除交易ID与交易签名相关的内容序列化，序列化结果Data
+ txid = SHA3-256(Data)
+ signature = Ed25519(from, Data)
+ 将txid与signature与序列化Bytes链接并序列化

#### 1.3 序列化协议：Protobuf

```protobuf
message Transaction {
	bytes to;
	bytes from;
	bytes operator;
	repeated bytes documents;
	bytes data;
}

message SignedTransaction {
	bytes txid;
	bytes signature;
	bytes txbytes;
}
```

#### 1.4. 交易验证流程：
+ 解析SignedTranscation，提取txbytes
+ 解析txbytes成为Transcation
+ 验证txid == SHA3-256(txbytes)
+ 验证ed25519_veryfy(signature, from, txbytes)

### 2. 区块数据：
#### 2.1 区块结构：

| 字段名      | 类型              | 描述         |
|-------------|-------------------|--------------|
| blockid     | SHA3-256          | 区块 ID      |
| signature   | Ed25519           | 区块签名     |
| timestamp   | UTC - Unix 时间戳 | 区块产生时间 |
| height      | uint64            | 区块高度     |
| miner       | PublicKey         | 区块生成者   |
| merkle_root | SHA3-256          | merkle 树根  |

#### 2.2 区块签名流程
+ 除区块ID与区块签名相关的内容序列化，序列化结果Data
+ blockid = SHA3-256(Data)
+ signature = Ed25519(from, Data)
+ 将txid与signature与序列化Bytes链接并序列化

#### 2.3 序列化协议：Protobuf

```protobuf
message BlockHeader {
	uint32 timestamp;
	uint64 height;
	bytes miner;
	bytes merkle_tree;
}

message SignedBlock {
	bytes blockid;
	bytes signature;
	bytes blockbytes;
}
```

#### 2.4 区块验证流程
+ 解析SignedBlock，提取blockbytes
+ 解析blockbytes成为Block
+ 验证blockid == SHA3-256(blockbytes)
+ 验证ed25519_veryfy(signature, miner, blockbytes)


