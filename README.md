# Lightcore

> Lightweight Blockchain-style Message Bus.

## Interface for control

These interface must be implement for control.

### Network Control

#### Get / Post Configuration

``` json
{
    "direct_node": [
        "192.168.1.1:3000"
    ],
    "port": 3870,
    "discovery_port": 3970,
    "cache_file": "/var/cache/lightcore/crust.dat",
    "network_id": "testnet",
    "whitelist": [
        "10.0.0.100"
    ]
}
```

#### Get Node Information

``` json
{
    "nodeid": "Nodeid"
}
```

### Block related

每一个共识周期在各个节点中产生的数据结构。

#### 区块结构

- 区块头
  - block_id
  - timestamp
  - height
  - miner
  - merkle_root
- 区块体
  - merkle_tree
  - transcation
- 交易
  - tx_id
  - object_id
  - from
  - to
  - nonce
  - operator
  - documents
    - doctype
    - content

``` mermaid
graph TD;
	New -- DataSubmit --> Created;
	Created -- PayFor --> PaymentComplete;
	Created -- Revert --> Reverted;
```



#### 用户发送交易流程

1. 准备参数（object_id, from, to, operator, documents）
2. 组成RawTranscation
3. 对RawTranscation签名
4. 发送JSONRPC (sendRawTranscation (data) )
5. 解析
6. 验证签名
7. 放入Tx池
8. 广播交易