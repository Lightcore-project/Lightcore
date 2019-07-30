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

### 交易流程图

``` mermaid
graph TD;
	subgraph javascript1
		prepare[准备参数] --> rawTranscation[组成rawTranscation];
		rawTranscation[组成rawTranscation] --> signTranscation[签名交易];
		signTranscation[签名交易] --> sendJSONRPC[发送JSONRPC];
	end
	sendJSONRPC --> ParseRawTranscation[解析rawTranscation];
	subgraph node1
		ParseRawTranscation --> VerifySignature[验证签名];
		VerifySignature --> TxPool1[交易池1];
		TxPool1 -- 提取交易 --> GenerateBlock[生成区块];
		GenerateBlock --提交共识--> Consensus1[共识器1];
	end
	subgraph node2
		VerifySignature --广播交易--> TxPool2[交易池2];
		Consensus1 -- 进行共识 --> Consensus2;
		Consensus2[共识器2] --> ParseBlock[解析区块];
		ParseBlock -- 提交交易 --> ExecuteTx[执行交易];
		ExecuteTx --> StatePool[状态池];
	end
	subgraph javascript2
		StatePool --> callJSONRPC[JSONRCP回调];
	end
```

