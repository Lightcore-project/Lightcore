# Lightcore

> Lightweight Blockchain-style Message Bus.

## Interface for control

These interface must be implement for control.

### Get / Post Configuration

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

### Get Node Information

``` json
{
    "nodeid": "Nodeid"
}
```


