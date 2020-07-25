# Lagoon

An inmemory and highly concurrent bloom filter service based on json-rpc.

[![Build Status](https://travis-ci.org/sourcepirate/lagoon.svg?branch=master)](https://travis-ci.org/sourcepirate/lagoon)

## Build 

```
# based on 32 bit vectors
cargo build 

# based on 16 bit vectors
cargo build --features 16
```

## Usage

For running locally
```
cargo run

# Open a new terminal
nc localhost 3030

```

## API Usage

```json

// initialize a new bloomfilter collection.
{"jsonrpc":"2.0","method":"createCollection", "params":["hello"], "id": 2}
{"jsonrpc":"2.0","result":true,"id":2}

// set a filter for a key.
{"jsonrpc":"2.0","method": "setKey", "params":["hello", "hi"], "id": 3}
{"jsonrpc":"2.0","result":true,"id":3}

// Check if the key exist.
{"jsonrpc":"2.0", "method": "hasKey", "params":["hello", "hi"], "id": 4}
{"jsonrpc":"2.0","result":true,"id":4}

// Check if the key exist.
{"jsonrpc":"2.0", "method": "hasKey", "params":["hello", "yo"], "id": 5}
{"jsonrpc":"2.0","result":false,"id":5}

```
