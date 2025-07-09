# Rust学習プロジェクト

このリポジトリは、Rustプログラミング言語の学習過程で作成したサンプルコードをまとめたモノレポです。

## プロジェクト構成

各章ごとに独立したCargoプロジェクト（クレート）として管理しています：

- `chapter_05_structs/` - 構造体を使ったプログラム例
- `chapter_06_enums/` - 列挙型（enum）の使用例
- （今後追加予定）

## 使用方法

### 全体のビルド
```bash
cargo build
```

### 特定のプロジェクトの実行
```bash
cargo run -p chapter_05_structs
```

### 特定のプロジェクトのテスト
```bash
cargo test -p chapter_05_structs
```

### ワークスペース全体のテスト
```bash
cargo test
```

## 開発環境

- Rust: 1.88.0
- IDE: RustRover
- OS: macOS

## 学習進捗

- [x] 第5章: 構造体
- [ ] 第6章: 列挙型とパターンマッチング
- [ ] 第7章: パッケージ、クレート、モジュール
