# Kasane Logic

**Kasane Logic** は、IPA（独立行政法人情報処理推進機構）が定義した 4 次元空間情報の記法を拡張し、時空間 ID に対する論理演算を可能にする Rust ライブラリです。演算は純粋な Rust の機能のみで実装されており、外部依存なくあらゆる環境で正確かつ高速に動作します。

[🇬🇧 English Version](./README.md)

## 🌱 特長

- `SpaceTimeId` による 4 次元（F, X, Y, T）空間の表現
- `DimensionRange` による範囲指定、無限範囲の柔軟な記述
- `SpaceTimeIdSet` による集合管理と重複排除
- 和集合（OR）、積集合（AND）、補集合（NOT）、排他的論理和（XOR）などの演算子サポート
- 実行環境に依存しない軽量な構成

## インストールのオプション

```toml
logic = { path = "../logic", features = ["serde_support"] }
```

`serde_support`を指定して、[`serde`](https://crates.io/crates/serde)と[`jsonschema`](https://crates.io/crates/jsonschema)に対応した型を出力します

## 📦 `SpaceTimeId` 型

`SpaceTimeId` は 4 次元空間（F, X, Y, T）に加え、ズームレベルと時間間隔を持つ領域を表します。

> [!NOTE]
> 時空間 ID は `i` が 0 ではない値を示し、空間 ID は `i` が 0 かつ `t` が `Any` です。

### 時空間 ID

```rust
let stid = SpaceTimeId::new(
    4,                                      // ズームレベル
    DimensionRange::AfterUnLimitRange(10),  // 高さ（f）: 10以上
    DimensionRange::Single(5),              // x座標
    DimensionRange::Single(3),              // y座標
    60,                                     // 時間間隔（秒）
    DimensionRange::LimitRange(100, 200),   // 時間インデックス（t）
).unwrap();
```

### 空間 ID

空間 ID は全ての時間において有効なオブジェクトを表します。

```rust
let stid = SpaceTimeId::new(
    4,                                      // ズームレベル
    DimensionRange::AfterUnLimitRange(10),  // 高さ（f）: 10以上
    DimensionRange::Single(5),              // x座標
    DimensionRange::Single(3),              // y座標
    0,                                      // i=0で空間IDを指定
    DimensionRange::Any,                    // 時間インデックスは全ての値を示すAny
).unwrap();
```

## ✍️ 記法

独自に以下の拡張記法を導入しています：

### 欠けた次元の表現

特定の次元に制約がない（すべての値が対象）場合は、`-` を使用します。

例：F 次元が未定義の場合

```
2/-/1/3
```

これは次の集合と等価です：

```
2/-4/1/3, 2/-3/1/3, ..., 2/3/1/3
```

### 範囲の指定

`:` を使って区間を示します：

```
2/1/3/1:5
```

これは以下と等価です：

```
2/1/3/1, 2/1/3/2, 2/1/3/3, 2/1/3/4, 2/1/3/5
```

### 以降すべての値

始点から無限大を意味するには `:` と `-` を併用します：

```
2/1/3/1:-
```

これは

```
2/1/3/1, 2/1/3/2, 2/1/3/3, ...
```

を意味します。

## 📐 `DimensionRange<T>` 型

各次元（F, X, Y, T）の値を表す汎用範囲型：

- `Single(v)`：単一の値
- `LimitRange(start, end)`：開始～終了の閉区間
- `BeforeUnLimitRange(end)`：値の下限から `end` まで
- `AfterUnLimitRange(start)`：`start` から値の上限まで
- `Any`：すべての値にマッチ（全領域）

---

## 🔧 `SpaceTimeId` の関数・メソッド

### 📍 座標取得関数

#### `coordinates() -> Coordinates`

SpaceTimeId を地理座標（緯度、経度、高度）に変換します。

```rust
let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3), DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
let coords = stid.coordinates();
println!("緯度: {:?}, 経度: {:?}, 高度: {:?}", coords.latitude, coords.longitude, coords.altitude);
```

#### `center() -> Point`

空間領域の中心点を返します。

```rust
let center = stid.center();
println!("中心点 - 緯度: {}, 経度: {}, 高度: {}", center.latitude, center.longitude, center.altitude);
```

#### `vertex() -> [Point; 8]`

空間領域の 8 つの角の頂点を返します。

```rust
let vertices = stid.vertex();
for (i, vertex) in vertices.iter().enumerate() {
    println!("頂点{}: 緯度={}, 経度={}, 高度={}", i, vertex.latitude, vertex.longitude, vertex.altitude);
}
```

### 🔄 スケール変換

#### `change_scale(z: Option<u16>, i: Option<u32>) -> Result<SpaceTimeId, String>`

空間解像度（ズームレベル）や時間解像度（時間間隔）を変更します。

```rust
// ズームレベルを6に変更
let scaled = stid.change_scale(Some(6), None)?;

// 時間間隔を30秒に変更
let time_scaled = stid.change_scale(None, Some(30))?;
```

### 🎯 包含関係

#### `containment_relation(&other: &SpaceTimeId) -> Containment`

他の SpaceTimeId との包含関係を判定します。

```rust
let containment = stid1.containment_relation(&stid2);
match containment {
    Containment::Full => println!("stid1 は stid2 を完全に含んでいます"),
    Containment::Partial(intersection) => println!("部分的に重複しています: {}", intersection),
    Containment::None => println!("重複していません"),
}
```

### ✂️ 補集合演算

#### `complement(&self) -> SpaceTimeIdSet`

この SpaceTimeId の補集合（含まれていない領域）を返します。

```rust
let complement_set = stid.complement();
println!("補集合: {}", complement_set);
```

### 🔍 純粋 ID 展開

#### `pure(&self) -> Vec<SpaceTimeId>`

空間次元（F、X、Y）の全ての範囲記法（Any、LimitRange、BeforeUnLimitRange、AfterUnLimitRange）を、Single 値のみを持つ個別の SpaceTimeId に展開します。時間次元（T）はそのまま保持されます。

この関数は、複雑な範囲ベースの SpaceTimeId を、精密な処理のための具体的で列挙された SpaceTimeId に変換する際に便利です。

```rust
// 範囲次元を持つSpaceTimeIdを作成
let stid = SpaceTimeId::new(
    2,                                    // ズームレベル2
    DimensionRange::LimitRange(0, 1),     // F次元: 0から1まで
    DimensionRange::LimitRange(1, 2),     // X次元: 1から2まで
    DimensionRange::Single(0),            // Y次元: 単一値0
    60,                                   // 時間間隔
    DimensionRange::Single(100),          // T次元: 単一値100
).unwrap();

// 純粋IDに展開
let pure_ids = stid.pure();
println!("{}個の純粋IDに展開されました", pure_ids.len()); // 4個のID（2個のF値 × 2個のX値 × 1個のY値）

// 各純粋IDはF、X、Y次元にSingle値のみを持ちます
for pure_id in pure_ids {
    println!("{}", pure_id); // 例: "2/0/1/0_60/100", "2/0/2/0_60/100", など
}
```

### 📊 値取得メソッド

各次元の値や属性にアクセスするためのゲッターメソッド：

- `f() -> DimensionRange<i32>`：F 次元（高度）の値
- `x() -> DimensionRange<u32>`：X 次元の値
- `y() -> DimensionRange<u32>`：Y 次元の値
- `t() -> DimensionRange<u32>`：T 次元（時間インデックス）の値
- `z() -> u16`：ズームレベル
- `i() -> u32`：時間間隔（秒）

---

## 🏗️ 補助型

### `Point` 構造体

3 次元空間内の点を表します。

```rust
pub struct Point {
    pub latitude: f64,   // 緯度
    pub longitude: f64,  // 経度
    pub altitude: f64,   // 高度
}
```

### `Coordinates` 構造体

空間領域の座標範囲を表します。

```rust
pub struct Coordinates {
    pub latitude: (f64, f64),   // 緯度の範囲 (最小, 最大)
    pub longitude: (f64, f64),  // 経度の範囲 (最小, 最大)
    pub altitude: (f64, f64),   // 高度の範囲 (最小, 最大)
}
```

### `Containment` 列挙型

包含関係の種類を表します。

```rust
pub enum Containment {
    Full,                    // 完全包含
    Partial(SpaceTimeId),   // 部分重複（交差領域を含む）
    None,                   // 重複なし
}
```

---

## 📚 `SpaceTimeIdSet` 型

複数の `SpaceTimeId` を集合として管理します。追加時に重複や重なりがある場合は自動的に調整され、物理的な空間に対して一意な ID のみが残ります。

### 🧭 基本的な使い方

```rust
let mut set = SpaceTimeIdSet::new();
set.insert(stid);
```

### 🛠️ `SpaceTimeIdSet` のメソッド

#### `new() -> SpaceTimeIdSet`

新しい空の集合を作成します。

```rust
let mut set = SpaceTimeIdSet::new();
```

#### `insert(&mut self, other: SpaceTimeId)`

SpaceTimeId を集合に追加します。重複や重なりがある場合は自動的に調整されます。

```rust
set.insert(stid);
```

#### `iter() -> impl Iterator<Item = &SpaceTimeId>`

集合内の要素を反復処理するイテレータを返します。

```rust
for id in set.iter() {
    println!("{}", id);
}
```

#### `is_empty() -> bool`

集合が空かどうかを確認します。

```rust
if set.is_empty() {
    println!("集合は空です");
}
```

#### `from(id: SpaceTimeId) -> SpaceTimeIdSet`

単一の SpaceTimeId から集合を作成します。

```rust
let set = SpaceTimeIdSet::from(stid);
```

### 🔀 対応演算子

- `|`：和集合 - 2 つの集合を結合
- `&`：積集合 - 共通部分を抽出
- `^`：排他的論理和（XOR）- どちらか一方の集合にのみ含まれる領域を返す
- `!`：補集合 - 含まれていない領域を返す
- `==`：等価比較 - 実体が同じ空間領域を示していれば true

```rust
let union = set_a | set_b;
let intersection = set_a & set_b;
let xor = set_a ^ set_b;
let complement = !set_a;
assert_eq!(set_a, set_b); // 物理的に等価であれば true
```

## 🧪 使用例

```rust
let a = SpaceTimeId::new(...).unwrap();
let b = SpaceTimeId::new(...).unwrap();

let mut set = SpaceTimeIdSet::new();
set.insert(a);

let set2 = SpaceTimeIdSet::from(b);
let union = set | set2;
let common = set & set2;
let outside = !set;
```

## 📋 API リファレンス

### `SpaceTimeId` コンストラクタ

- `new(z: u16, f: DimensionRange<i32>, x: DimensionRange<u32>, y: DimensionRange<u32>, i: u32, t: DimensionRange<u32>) -> Result<SpaceTimeId, String>`

### `SpaceTimeId` インスタンスメソッド

- `coordinates() -> Coordinates` - 地理座標を取得
- `center() -> Point` - 空間領域の中心点を取得
- `vertex() -> [Point; 8]` - 8 つの角の頂点を取得
- `change_scale(z: Option<u16>, i: Option<u32>) -> Result<SpaceTimeId, String>` - 解像度を変更
- `containment_relation(&other: &SpaceTimeId) -> Containment` - 包含関係を確認
- `complement() -> SpaceTimeIdSet` - 補集合を取得
- `pure() -> Vec<SpaceTimeId>` - 範囲次元を個別の SpaceTimeId に展開
- `with_z(z: u16) -> Result<SpaceTimeId, String>` - 異なるズームレベルで新しい ID を作成
- `with_f(f: DimensionRange<i32>) -> Result<SpaceTimeId, String>` - 異なる F 次元で新しい ID を作成
- `with_x(x: DimensionRange<u32>) -> Result<SpaceTimeId, String>` - 異なる X 次元で新しい ID を作成
- `with_y(y: DimensionRange<u32>) -> Result<SpaceTimeId, String>` - 異なる Y 次元で新しい ID を作成
- `with_i(i: u32) -> Result<SpaceTimeId, String>` - 異なる時間間隔で新しい ID を作成
- `with_t(t: DimensionRange<u32>) -> Result<SpaceTimeId, String>` - 異なる T 次元で新しい ID を作成
- `f() -> DimensionRange<i32>` - F 次元の値を取得
- `x() -> DimensionRange<u32>` - X 次元の値を取得
- `y() -> DimensionRange<u32>` - Y 次元の値を取得
- `t() -> DimensionRange<u32>` - T 次元の値を取得
- `z() -> u16` - ズームレベルを取得
- `i() -> u32` - 時間間隔を取得

### `SpaceTimeIdSet` メソッド

- `new() -> SpaceTimeIdSet` - 新しい空の集合を作成
- `from(id: SpaceTimeId) -> SpaceTimeIdSet` - 単一 ID から集合を作成
- `insert(&mut self, other: SpaceTimeId)` - ID を集合に追加
- `iter() -> impl Iterator<Item = &SpaceTimeId>` - イテレータを取得
- `is_empty() -> bool` - 集合が空かを確認
- `pure(&self) -> Vec<SpaceTimeId>` - 集合内の全要素を純粋形式に展開

### `SpaceTimeIdSet` 演算子

- `set1 | set2` - 和集合演算
- `set1 & set2` - 積集合演算
- `set1 ^ set2` - XOR（排他的論理和）演算
- `!set` - 補集合演算
- `set1 == set2` - 等価比較

## 🤝 コントリビューションについて

バグ報告・機能提案・ドキュメント修正・テスト追加など、あらゆる貢献を歓迎します。

### 開発手順

1. 本リポジトリを fork する
1. fork したリポジトリを clone する
1. ローカルで作業用 branch を作成
   - ブランチ名は`<種別>/<短い説明>(-<issue番号>)`issue 番号は issue が存在する場合
   - 種別
     - `feat` → 新機能
     - `fix` → バグ修正
     - `docs` → ドキュメント修正
     - `refactor` → リファクタリング
     - `chore` → 雑務（CI 設定、依存パッケージ更新など）
   - 例:`feat/add-user-login-123`
1. 開発・commit
   - commit メッセージは任意の短い説明
1. GitHub に push
1. Pull Request
   - タイトルはブランチ名を参照し、`[種別] #<issue番号> 短い説明`issue 番号は issue が存在する場合

## 🧪 テストに関する考え方

このライブラリは正確性を第一として作成します。よって、テストは最大限充実する方針で整備します：

- コードを改善したときに以前のテストと異なる挙動になった場合にバグの発生なのか修正なのかを議論します
- とにかくたくさんのテストを充実させることで挙動が変わった場合に検知できるようになります
- `cargo test`で実行できます

## ⚡ パフォーマンステストに関する考え方

関数のパフォーマンスが高くなるように改良を行います：

- `cargo bench`で実行できます
- criterion を用いてテストを行います
- まだ全ての関数を網羅できていません
