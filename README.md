# ULID Generator MCP Server

Model Context Protocol (MCP)を利用したULID（Universally Unique Lexicographically Sortable Identifier）を生成するためのサーバーです。

[rmcp](https://github.com/modelcontextprotocol/rust-sdk)を利用しています。


## 機能

- MCPプロトコルを使用してULIDを生成

## 必要条件

- Rust 1.75以上

## インストール

```bash
git clone https://github.com/yutak03/ulid-gen-mcp-rs.git
```

## 使い方

ビルドする。

```bash
cd ulid-gen-mcp-rs
cargo build --release
```

パーミッションを実行できる形にしておく

```bash
chmod +x ./target/release/ulid_gen_server
```

### 設定ファイルを記載する

mcpの設定を記載し、書いた設定ファイルをClaude desktopの設定に追加する。

```json
{
  "mcpServers": {
    "ulid-gen-server": {
      "command": "/path/to/ulid-gen-mcp-rs/target/release/ulid_gen_server",
      "args": []
    }
  }
}
```
