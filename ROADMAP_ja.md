# cgmath-next Roadmap

> **Keep the API. Remove the unsoundness. Build the safest path forward.**

## 現在の状況 (2026-08-16)

`0.18.1-alpha.1`実装フェーズ。下記「0.18.1 — Safe Successor」に明記された
完了条件5件、およびAGENTS.md §20のalpha gate（12条件）はすべて満たされている
（詳細は`docs/release-checklist.md`参照）。**ただしこれは「0.18.1が完成した」
という意味ではない** -- 同じ`docs/release-checklist.md`のstable gateでは
`UNSAFE-002`が未解決のため12条件中1件が引き続きnot met であり、
`0.18.1-alpha.1`は名前の通りalpha候補の段階に留まる。

* 既知の`swap_columns` UBを再現でき、修正後はMiriで成功する -- met
  (`tests/soundness/`、`cargo +nightly miri test --test soundness`で22/22)
* upstream由来テストが成功する -- met (256/256、ソース変更なし)
* public API差分に未説明項目がない -- met (`docs/api-inventory.md`、差分ゼロ)
* 代表的な逆依存crate 5件以上が移行できる -- met
  (`arcball`/`crevice`/`truck-base`/`vector-traits`/`three-d`、
  `compat/fixtures/reverse-deps/RESULTS.md`)
* すべてのunsafe使用に監査状態が付いている -- met (`docs/unsafe-audit.md`)

「0.18.2 — Soundness Closure」以降は未着手。特に`UNSAFE-002`
（tuple-transmute layout、`docs/unsafe-audit.md`参照）は0.18.1時点で解消の
見込みがなく、public API変更を伴わない限り0.18.2以降でも根本解決は難しい
可能性がある -- 詳細は同docの「External corroboration」節（upstream側の
`rustgd/cgmath#538`も2021年から未解決）を参照。

`crates.io` publish、git tag、GitHub Releaseはいずれも未実施（AGENTS.md
§21のstop-and-report対象、人間の明示承認が必要）。

## Mission

`cgmath-next`は、`cgmath` 0.18.0の主要public API、型、数値規約、データ表現を可能な限り維持しながら、安全性、保守性、相互運用性、実行性能、開発体験を段階的に向上させるコミュニティ主導の後継ライブラリです。

単なる保存用forkにはしません。

目標は、次の条件を同時に満たすことです。

1. 既存`cgmath`利用者にとって最も移行しやすい選択肢である
2. safe Rustから未定義動作へ到達させない
3. グラフィックス、ゲーム、シミュレーション向けに十分な性能を持つ
4. GPU、serialization、他のmath crateとの接続点を広く持つ
5. API、数値規約、メモリレイアウトについて予測可能である
6. 少人数でも長期保守できる規模と設計を維持する

---

## Competitive Position

2026年8月時点の主要競合には、異なる強みがあります。

* `glam`は固定型、SIMD、`no_std`、GPU関連、serialization、zero-copy関連の豊富なfeatureを持ち、ゲーム・グラフィックス用途で強い
* `nalgebra`は静的・動的次元、高度な行列分解、疎行列、幅広い数値計算を扱う
* `ultraviolet`はscalar型と明示的なwide型を分け、SoA形式で複数データを並列処理する
* `vek`はゲームエンジン向けの広い便利機能を提供する
* `euclid`は座標空間や単位を型で区別する安全性に強みを持つ

`cgmath-next`は、これらすべてを模倣しません。

### 勝つ領域

`cgmath-next`が狙う立ち位置は次のとおりです。

> **The safest and most interoperable generic graphics-math library for existing cgmath users.**

具体的には、以下の組み合わせで差別化します。

* `cgmath` 0.18との高いソース互換性
* generic scalar型を維持したgraphics-oriented API
* 全unsafeコードの監査可能性
* 安全な失敗を表現するrobust API
* `glam`、`nalgebra`、`mint`、GPU関連crateとの相互変換
* optionalかつ明示的な高速化
* 軽量なcoreと、用途別extensionの分離
* 数値規約とデータレイアウトの明文化

### 追わない領域

以下では競合しません。

* `nalgebra`と同等の一般数値線形代数
* 動的次元行列
* 大規模な疎行列演算
* LAPACK代替
* 自動微分
* GPU compute framework
* 物理エンジン
* 幾何カーネル
* 独自serialization framework
* `glam`と同じ固定型APIへの全面移行

---

# Compatibility Policy

## 互換性の層

互換性を一語で扱わず、次の6層に分けて管理します。

| 層                           | 対象                         | 方針                   |
| --------------------------- | -------------------------- | -------------------- |
| Source compatibility        | 既存Rustコードの再コンパイル           | 最優先で維持               |
| Behavioral compatibility    | 演算結果、panic、特殊値             | soundnessを損なわない範囲で維持 |
| Feature compatibility       | `serde`、`mint`、`rand`等     | 既存featureを維持         |
| Data-layout compatibility   | `size_of`、alignment、field順 | 明示的に測定・固定            |
| Serialization compatibility | JSON等の表現                   | 原則維持                 |
| ABI compatibility           | コンパイル済みbinaryとの互換          | 保証しない                |

## Version policy

### `0.18.x`

`cgmath` 0.18.0からの移行を最優先するstrict compatibility系列です。

許可する変更：

* soundness修正
* bug fix
* documentation改善
* additive API
* additive optional feature
* 内部実装の安全化
* 性能改善
* toolchain対応

原則として許可しない変更：

* public item削除
* public field変更
* trait bound強化
* matrix layout変更
* serialization形式変更
* 演算規約変更
* default feature追加による依存増加

### `0.19`以降

原則としてsource compatibilityを維持しますが、将来の1.0へ向けた改善を段階的に導入できます。

破壊的変更を行う場合は、次を必須とします。

1. 既存APIを先にdeprecatedにする
2. 代替APIを少なくとも1 minor系列提供する
3. migration guideを用意する
4. public API diffを公開する
5. 機械的置換が可能ならmigration toolを提供する
6. 変更理由と利益を定量的に説明する

## Compatibility budget

各minor releaseで許容する非互換変更には上限を設けます。

* 意図しないpublic API差分：0件
* 未文書化された挙動変更：0件
* serialization破壊：0件
* layout破壊：0件
* 意図的なsource incompatibility：原則0件
* やむを得ない変更：RFCと明示承認が必要

---

# Release Roadmap

## 0.18.1 — Safe Successor

### 目的

`cgmath` 0.18.0から安全に移行できる最初のリリースを作ります。

### 主な内容

* `cgmath` 0.18.0公開版をbaselineとして固定
* RustSecで報告された`swap_columns`のsoundness問題を修正
* upstreamテストを仕様コーパス化
* public API inventoryの作成
* unsafeコードの全件inventory
* `serde`、`mint`、`rand`、`swizzle`の互換性確認
* dependency renameによる移行方法を検証
* Linux、Windows、macOS CI
* Miriによるtargeted soundness suite
* `SECURITY.md`、`CHANGELOG.md`、migration guide
* MSRVの実測と宣言

### 完了条件

* 既知の`swap_columns` UBを再現でき、修正後はMiriで成功する
* upstream由来テストが成功する
* public API差分に未説明項目がない
* 代表的な逆依存crate 5件以上が移行できる
* すべてのunsafe使用に監査状態が付いている
* crates.io公開前のrelease checklistが完成している

---

## 0.18.2 — Soundness Closure

### 目的

既知の1件だけでなく、crate全体をsoundness-firstで監査します。

### 主な内容

* raw pointer、unchecked indexing、aliasingの重点監査
* safe Rustで置換できるunsafeの削除
* 全残存unsafeへの具体的な`SAFETY:`コメント
* Miri test範囲の拡大
* property testの導入
* fuzz targetの最小導入
* panic、NaN、infinity、zero-length vector、singular matrixの挙動整理
* `#[must_use]`の追加候補調査
* invalid inputに関するdocumentationの統一

### Acceptance targets

* safe APIから到達可能な既知UB：0件
* 未監査unsafe block：0件
* soundness regression test：全件成功
* fuzzingで得られたcrash、panic、UB候補：分類済み
* 新規unsafe：0件を原則とする

---

## 0.18.3 — Compatibility Proof

### 目的

「互換だと思う」状態から、「どこまで互換か測定できる」状態へ移行します。

### 主な内容

* rustdoc JSONまたは同等手段によるAPI diff CI
* 旧`cgmath`とのdifferential testing
* layout snapshot
* serialization snapshot
* feature matrixの自動検証
* doctest compatibility
* 逆依存fixtureの拡大
* compatibility dashboardの生成
* known differences一覧の公開

### Acceptance targets

* public API coverage：100%
* 代表的な数値演算のdifferential test：100%成功
* 逆依存fixture：10件以上
* ソース変更なしで移行可能なfixture：80%以上
* 未分類のbehavioral difference：0件

---

## 0.19 — Modern Interoperability

### 目的

互換性を保ったまま、現代のRustグラフィックス環境へ接続しやすくします。

### Feature candidates

すべてoptional featureとして検討します。

* `bytemuck`
* `zerocopy`
* `encase`
* `rkyv`
* `arbitrary`
* `proptest-support`
* 更新された`mint`
* `glam`変換
* `nalgebra`変換

`glam`は`bytemuck`、`encase`、`rkyv`、`serde`、`zerocopy`など幅広いoptional integrationを提供しています。`cgmath-next`も依存をcoreへ強制せず、利用者が必要な接続だけ選べる構成を目指します。

### Design rules

* default featureは最小限に保つ
* optional dependencyをpublic core APIへ漏らさない
* feature combination explosionを抑える
* conversionは意味が一意な場合だけ実装する
* matrix orientationとquaternion component orderを必ず文書化する
* zero-copy traitはlayout検証後にのみ実装する

### GPU interoperability

以下の用途を重視します。

* `wgpu`
* `encase`
* uniform/storage buffer
* vertex attributes
* column-major array export
* row-major export
* packed array conversion
* explicit alignment diagnostics

新しいGPU専用matrix型をcoreへ追加するのではなく、安全な変換層を優先します。

### Acceptance targets

* optional integrationがdefault buildへ影響しない
* all-featuresおよび主要feature pairがCIで成功する
* conversion round-trip testが成功する
* layout-sensitive integrationは全対象platformで検証される

---

## 0.20 — Robust Graphics Math

### 目的

従来APIを壊さず、「失敗し得る演算を安全に扱える」新しいAPIを追加します。

### Additive API candidates

* `try_normalize`
* `try_inverse`
* `try_look_at`
* `try_from_axis_angle`
* `try_from_basis`
* checked projection constructors
* finite-value validation
* orthonormality validation
* determinant threshold policy
* explicit epsilon variants
* normalized wrapper types
* unit quaternion wrapperまたはvalidated constructor
* affine transform validation

例：

```rust
let unit = vector.try_normalize()?;
let inverse = matrix.try_inverse()?;
```

既存の`normalize`や`invert`は削除しません。

### Error design

core APIを重くしないよう、エラーは小さく、比較可能で、`no_std`でも利用可能な型にします。

候補：

```rust
pub enum MathError {
    ZeroLength,
    NonFinite,
    Singular,
    DegenerateBasis,
    InvalidProjection,
}
```

実際の公開形はAPI review後に決定します。

### 数値ポリシー

以下を明文化します。

* exact zeroとepsilon判定の違い
* NaNの伝播
* infinityの扱い
* singular判定
* normalization threshold
* handedness
* depth range
* column-major storage
* column-vector convention
* quaternion component order

### Acceptance targets

* checked APIはpanicを通常の失敗表現に使用しない
* 既存APIの挙動は変更しない
* checked/unchecked APIの対応表を公開する
* degenerate case testを十分に持つ
* `no_std`環境でもchecked APIが利用できる

---

## 0.21 — Typed Spaces

### 目的

world、view、screen、modelなど異なる座標空間の混同を、optionalな型安全APIで防止します。

`euclid`はgenericなunit parameterによって、異なる座標空間のpointやvectorをコンパイル時に区別します。`cgmath-next`では既存型のgeneric parameterを変更せず、新しいwrapperまたは別moduleとしてこの考え方を導入します。

### Design constraints

次は禁止します。

* `Vector3<S>`を`Vector3<S, Space>`へ変更する
* `Point3<S>`のgeneric arityを変更する
* 既存`Transform` traitの破壊
* core利用者へspace markerを強制する

### Candidate design

```rust
struct World;
struct View;
struct Clip;

type WorldPoint = space::Point3<f32, World>;
type WorldToView = space::Transform3<f32, World, View>;
```

### Required capabilities

* typed point
* typed vector
* typed scale
* typed affine transform
* source/destination spaceを持つtransform
* untyped既存型との明示変換
* zero-cost abstractionの検証
* serdeとGPU出力ではmarker typeを保存しない

### Acceptance targets

* runtime size増加：0
* optimized assembly上の追加コスト：0
* 既存core APIへの破壊：0
* world/view/clipの混同をcompile-fail testで検出
* migrationは完全にoptional

---

## 0.22 — Performance Foundation

### 目的

互換性とgeneric APIを維持しながら、不要なperformance gapを縮小します。

### 方針

`glam`はSIMDを利用する固定型設計を持ち、`ultraviolet`は複数の値を同時処理する明示的なwide型を提供します。`cgmath-next`は既存型のlayoutを変えて追随するのではなく、scalar coreとbatch accelerationを分離します。

### Phase 1: scalar optimization

* bounds-check削減
* unnecessary copy削減
* iteratorとmanual expressionの比較
* matrix multiplicationのコード生成確認
* inverse、determinant、quaternion演算の最適化
* `#[inline]`方針の再評価
* compile-timeとruntimeのバランス測定
* platform間の数値再現性確認

### Phase 2: explicit batch API

新しいoptional moduleとして検討します。

```rust
cgmath::batch
```

候補：

* `Vector3x4`
* `Vector3x8`
* `Quaternionx4`
* `Matrix4x4`
* slice-based transform operations
* bulk normalize
* bulk dot product
* bulk point transformation

wide型を導入する場合でも、既存`Vector3`や`Matrix4`のlayoutは変更しません。

### Phase 3: portable acceleration

検討順序：

1. 安全なstable Rust
2. compiler auto-vectorization
3. 十分に保守されたsafe SIMD abstraction
4. 将来安定したportable SIMD
5. target-specific unsafeは最終手段

### Performance gates

以下を定期測定します。

* clean compile time
* incremental compile time
* binary size
* `Vector3` dot/cross/normalize
* `Matrix3` multiplication
* `Matrix4` multiplication
* `Matrix4` inverse
* quaternion multiplication
* quaternion-vector rotation
* bulk point transform
* WASM scalar performance

### Target policy

* 0.18 baselineに対する10%以上のregressionを原則禁止
* 最適化は数値互換性testを通過すること
* benchmark結果なしに「高速」と主張しない
* `glam`や`ultraviolet`に勝つという表現は、同一条件の公開benchmarkで確認された操作に限定する
* benchmarkは毎コミットではなく、性能関連PRとrelease candidateで実行する

---

## 0.23 — `no_std` and Embedded Graphics

### 目的

デスクトップ以外でも利用しやすいgraphics math coreを作ります。

### 主な内容

* `std`依存の分離
* `libm` integration
* allocator不要のcore確認
* embedded target CI
* WASM/WASI CI
* `no_std` feature matrix
* panic-free checked API
* serialization integrationの`no_std`対応範囲明示

`glam`はdefault featureを無効化し、`libm`を利用する`no_std`構成を提供しています。`cgmath-next`も同様に、利用者が`std`と`libm`を明示的に選べる構成を目指します。

### Acceptance targets

* core typesがallocatorなしで利用できる
* `thumbv7em-none-eabihf`等の代表targetでcheck成功
* `wasm32-unknown-unknown`でcheck/test可能な範囲を自動検証
* `std`無効時に不要なdependencyを引き込まない

---

## 0.24 — Developer Experience

### 目的

性能や機能だけでなく、導入・理解・デバッグのしやすさで競争力を持たせます。

### Documentation

* cgmathからの5分移行guide
* matrix convention guide
* transform composition guide
* quaternion guide
* camera/projection guide
* GPU upload guide
* `no_std` guide
* checked API guide
* typed spaces guide
* competitor migration guide
* common mistakes
* cookbook examples

### Diagnostics

optionalなvalidation featureを検討します。

```toml
cgmath-next = {
    version = "...",
    features = ["debug-math-assert"]
}
```

検出候補：

* zero-length normalize
* non-finite component
* singular inverse
* invalid projection parameters
* non-unit quaternion
* degenerate look-at direction
* non-orthonormal basis

release buildへ強制的なコストを追加しない設計にします。

### Tooling

* API diff report
* compatibility report
* layout report
* benchmark report
* dependency feature visualizer
* migration lintsまたは`cargo fix`候補
* examplesをCIでcompile
* documentation link check

---

## 0.25 — Ecosystem Adoption

### 目的

技術的に良いだけでなく、実際に移行されるライブラリへ育てます。

### 主な活動

* 逆依存上位crateへの移行PR
* game engine、renderer、geometry crateとの検証
* migration case study
* crates.io metadataと検索語の改善
* `Are We Game Yet?`等のecosystem掲載候補調査
* RustSec advisoryのpatched successorとしての認知形成
* independent auditの募集
* contributor guideの改善
* good first issue整備
* release cadenceの安定化

### Adoption targets

1. crates.io reverse dependency：10件
2. 月間download：旧cgmath利用者の移行を測れる水準
3. 代表的な実利用プロジェクト：3件
4. 外部maintainerまたはreviewer：2名以上
5. releaseを単独maintainerだけに依存しない体制

数値は観測開始後に現実的な基準へ更新します。

---

# 1.0 Release Criteria

`1.0`は機能数ではなく、契約の安定性によって判断します。

以下をすべて満たすまで1.0にしません。

## Compatibility

* `cgmath` 0.18からのmigration guideが完成している
* 主要public APIが少なくとも2 minor系列安定している
* data layout policyが確定している
* serialization policyが確定している
* feature policyが確定している
* deprecation policyが運用実績を持つ

## Safety

* safe APIから到達可能な既知UBがない
* 全unsafeが監査済み
* Miri、property test、fuzzingがrelease processへ組み込まれている
* security reportingとadvisory processが確立している

## Quality

* public item documentation coverageが十分である
* 全主要platformのCIが安定している
* MSRV policyが運用されている
* benchmark historyが保存されている
* known limitationsが公開されている

## Ecosystem

* 代表的な実利用プロジェクトで利用されている
* 単一人物にしかreleaseできない状態ではない
* 重大issueへの対応方針が明文化されている
* 少なくとも1回の外部reviewまたはauditを受けている

---

# Success Metrics

## Safety

| 指標              |     目標 |
| --------------- | -----: |
| 既知のsafe-to-UB経路 |      0 |
| 未監査unsafe block |      0 |
| 新規unsafe        |    原則0 |
| Miri regression | 100%成功 |
| 未分類fuzz crash   |      0 |

## Compatibility

| 指標                       | 0.18.3目標 | 1.0目標 |
| ------------------------ | -------: | ----: |
| Public API inventory     |     100% |  100% |
| 未説明API差分                 |        0 |     0 |
| Fixture無変更移行率            |    80%以上 | 95%以上 |
| Serialization regression |        0 |     0 |
| Layout regression        |        0 |     0 |

## Performance

| 指標                     |        目標 |
| ---------------------- | --------: |
| Baseline比10%以上の意図しない低下 |        0件 |
| Benchmark対象操作          |      10以上 |
| Performance claim      | 再現可能な測定付き |
| Benchmark実行            |  性能PR・RC時 |

## Maintenance

| 指標                     |        目標 |
| ---------------------- | --------: |
| Critical issue初期triage |      7日以内 |
| Security report初期確認    | 72時間以内を目標 |
| 未分類API差分               |         0 |
| Release checklist実施率   |      100% |
| Bus factor             |       2以上 |

これらは保証時間ではなく、プロジェクト運営上の目標です。

---

# Decision Framework

新機能を追加する前に、以下を確認します。

1. graphics math libraryとして必要か
2. 既存`cgmath`利用者に価値があるか
3. optional extensionとして実装できるか
4. public APIを不必要に複雑化しないか
5. unsafeを増やさず実装できるか
6. maintenance costを正当化できるか
7. 競合crateとの相互運用で解決できないか
8. 独自実装する明確な利点があるか
9. correctness testを定義できるか
10. 将来削除したくなる実験的APIではないか

3項目以上に明確な回答がない場合、その機能はcoreへ追加しません。

---

# Release Discipline

## 通常release

* 小さく焦点を絞る
* public API diffを添付する
* compatibility reportを添付する
* benchmarkは該当変更時だけ実行する
* security関連変更は独立して記録する
* feature追加はdefault無効から開始する
* release前にfixtureを固定条件で実行する

## Experimental API

実験的機能は以下のいずれかで隔離します。

* optional feature
* `experimental` module
* separate companion crate
* prerelease version

実験APIを通常のpreludeへ追加しません。

## Companion crates

coreが肥大化する場合は、次のような分離を検討します。

```text
cgmath-next
cgmath-next-spaces
cgmath-next-gpu
cgmath-next-batch
cgmath-next-interop
```

ただし、crate分割自体を目的化せず、feature graph、compile time、maintenance boundaryに明確な利益がある場合だけ実施します。

---

# Strategic Priority

開発優先度は常に次の順序とします。

1. Soundness
2. Compatibility
3. Correctness
4. Maintenance
5. Interoperability
6. Developer experience
7. Performance
8. New features

性能改善や新機能のために、上位項目を犠牲にしません。

---

# Summary

`cgmath-next`は、`glam`のコピーにも、縮小版`nalgebra`にもなりません。

目指すのは次の立場です。

> **既存cgmath利用者が安心して移行でき、型の汎用性を保ち、現代のgraphics ecosystemと広く接続できる、安全性重視の数学基盤。**

最初の勝利条件は、最速になることではありません。

最初の勝利条件は、`cgmath`利用者が次の1行だけで安全な後継へ移行できることです。

```toml
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

その互換基盤を守りながら、robust API、typed spaces、GPU interop、`no_std`、明示的なbatch accelerationを順番に積み上げ、1.0へ到達します。
