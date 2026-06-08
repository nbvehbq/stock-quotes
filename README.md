# Market Quotes

The project contains a library and two applications.

build:

```bash
cargo build
```

## Quote server

Server - generates quotes and sends them to the subscriber via UDP.

Run server:
```bash
server --addr=127.0.0.1:3000 --interval=1000 --tickers=./tickers.txt
```

all arguments are optional. By default server starts at localhost port 3000
with interval generator interval = 1000 ms and with `AAPL`, `MSFT`, `TSLA` tickers.

## Quote client

The client subscribes to specific quotes and receives them from the server via UDP.

Run client:
```bash
client --tcp-addr=127.0.0.1:3000 udp-port=12345 --tickers=./client.txt
```

All arguments are mandatory.
