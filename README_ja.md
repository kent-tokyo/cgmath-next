# cgmath-next

[![CI](https://img.shields.io/github/actions/workflow/status/kent-tokyo/cgmath-next/ci.yml?branch=main&label=CI)](https://github.com/kent-tokyo/cgmath-next/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/docsrs/cgmath-next)](https://docs.rs/cgmath-next)
[![crates.io](https://img.shields.io/crates/v/cgmath-next.svg)](https://crates.io/crates/cgmath-next)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/kent-tokyo/cgmath-next/blob/main/LICENSE)

[English](README.md) | 日本語 | [中文](README_zh.md)

`cgmath` 0.18 系列を保守し続ける、soundness(健全性)を重視したソース互換の後継クレートです。

> APIはそのまま、unsoundnessだけを取り除く。

`cgmath-next`は、[`cgmath`](https://github.com/rustgd/cgmath) 0.18.0を基に独立して開発される、コミュニティ運営の互換後継ライブラリです。

オリジナルの`cgmath`は保守が止まっており
([RUSTSEC-2026-0196](https://rustsec.org/advisories/RUSTSEC-2026-0196.html))、
既知のsoundness問題も抱えています
([RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html) /
[rustgd/cgmath#565](https://github.com/rustgd/cgmath/issues/565))。
`cgmath-next`は、この問題を修正し、soundnessおよび保守上の修正を継続して受け取れるようにするために存在します。可能な限りdrop-in(そのまま差し替え可能)な互換性を保ちます。

## 移行方法

多くの場合、移行は`Cargo.toml`の1行変更だけで完了します。

```toml
[dependencies]
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

```rust
use cgmath::{Matrix4, Quaternion, Vector3}; // 変更不要
```

これは`cgmath-next`のコンパイル後ライブラリ名が引き続き`cgmath`であるためです(この点をfixtureベースで検証した内容は[`docs/compatibility.md`](docs/compatibility.md)を参照)。詳細と、renameすら不要なケースについては[`docs/migration.md`](docs/migration.md)を参照してください。

## サンプル

カメラのview-projection行列を作成し、点をclip spaceおよびnormalized
device coordinatesへ変換する最小限の実行可能なサンプルは、
[`examples/camera_transform.rs`](examples/camera_transform.rs)を参照して
ください。レンダラーが頂点ごとに実行するのと同じ処理です。

```
cargo run --example camera_transform
```

## このプロジェクトが維持しているもの

* `cgmath` 0.18.0(crates.io公開版)のpublic API — 機械生成のpath diffで検証済みで、現時点で**差分ゼロ**([`docs/api-inventory.md`](docs/api-inventory.md)参照)
* 数値的な計算結果。soundness修正で変更が必要だった箇所を除く — 実際の`cgmath` 0.18.0に対する9ケースのdifferential testで、近似ではなく厳密な等価性を検証済み([`docs/compatibility.md`](docs/compatibility.md)参照)
* 型とtrait実装。`serde`/`mint`/`rand`のderive実装を含む — コード自体は変更していませんが、そのwire-format出力についてはupstreamに対する独立したround-tripテストはまだ行っていません(同じく`docs/compatibility.md`に記載)
* 元のクレートが既に宣言していた`[lib] name = "cgmath"` — 既存の`use cgmath::...`コードはそのままコンパイルできます
* Apache-2.0ライセンスと、オリジナルの著作権表示

## このプロジェクトが保証していないもの

* **「100%メモリセーフ」または「完全に監査済み」という主張ではありません。** soundnessに関する作業は継続中で、[`docs/unsafe-audit.md`](docs/unsafe-audit.md)で項目ごとに追跡しています。既存のunsafeコードの一部(`AsRef`/`AsMut`/`From`による、同種のtupleへの変換。例: `(f32, f32, f32)`)は、Rust言語仕様が正式には保証していないtupleのメモリレイアウトに依存しています — `UNSAFE-002`として記録されています。現在はこれを**ガード**しています。各変換の前にサイズ・アライメント・各フィールドのバイトオフセットを実行時に検証し、一致しない場合はtransmuteせずにpanicします(Miri、negative-controlテスト、レイアウトが一致する現状ではコストゼロであることを確認したrelease buildのdisassemblyで検証済み)。これにより、将来レイアウトが乖離した場合の挙動が、サイレントなundefined behaviorから、即座に検知できるpanicへと変わります。ただし、これは**言語レベルのsoundness証明ではありません** — tupleのレイアウトは公式には未規定のままであり、これを完全に解消する唯一の方法は参照を返す変換を削除すること(public APIの変更であり、このシリーズのスコープ外)です。詳細は`docs/unsafe-audit.md`のfeasibility study章、また本forkに固有の問題ではないことを裏付ける独立した情報として[`rustgd/cgmath#538`](https://github.com/rustgd/cgmath/issues/538)を参照してください。
* **Rust ABI互換性は保証していません。** `#[repr(C)]`のレイアウト(`size_of`、`align_of`、フィールドオフセット)は、それを宣言している型についてのみ検証されており、クレート全体に及ぶものではありません。
* **あらゆる用途に対する「完全なdrop-in置き換え」ではありません。** 検証済みの互換性は、実際にテストした範囲に限られます: upstreamのテストスイート(無変更、全件成功)、いくつかのdependency-rename fixture、そして5件の実際のdownstreamクレート([`compat/fixtures/reverse-deps/RESULTS.md`](compat/fixtures/reverse-deps/RESULTS.md)参照)。具体的に何を確認したかは[`docs/compatibility.md`](docs/compatibility.md)を参照してください。

## RustSec advisoryの状況

| Advisory | 状況 |
|---|---|
| [RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)(soundness: `swap_columns`の同一インデックスUB) | **修正済み**。advisoryの文字通りの範囲より広く修正しています — advisoryには明記されていない`Array::swap_elements`と`Matrix::swap_elements`(同じバグを共有)についても、[`docs/unsafe-audit.md`](docs/unsafe-audit.md)とプロジェクトのcommit履歴を参照してください。 |
| [RUSTSEC-2026-0196](https://rustsec.org/advisories/RUSTSEC-2026-0196.html)(unmaintained) | このプロジェクトの存在そのものが対応です: `cgmath-next`は現在も活発に保守されています。 |

## upstreamとの関係

`cgmath-next`は、upstreamの`master`ブランチ(未リリースの機能や依存関係の更新で乖離しています)ではなく、crates.ioで公開された`cgmath` 0.18.0を基準にしています。正確なソース、チェックサム、0.18.0タグ以降の各upstream commitの分類については[`docs/provenance.md`](docs/provenance.md)を参照してください。これまでに公開した内容は[公開状況](#公開状況)を参照してください。

## 互換性ポリシー

* public APIの削除・リネーム・シグネチャ変更・trait boundの強化は、0.18.1シリーズにおいてはリリースブロッカーとして扱い、密かに紛れ込ませることはしません。
* soundness修正と厳密な挙動維持が衝突する場合は、soundness修正を優先します(例: 同一インデックスのswapが明示的なno-opになる、など)。
* 新機能や大規模な再設計は、API-parityリリースの後まで延期します。プロジェクトが従う方針の全体像は、このリポジトリの`AGENTS.md`を参照してください。

## MSRV

計測された最小サポートRustバージョンと、その決定方法については[`docs/msrv.md`](docs/msrv.md)を参照してください。

## 機能(Features)

明示的なfeatureと暗黙のfeature(Cargoは各optional dependencyに対して自動的にfeatureを生成します)— 詳細な内訳は[`docs/compatibility.md`](docs/compatibility.md)を参照してください:

| Feature | 内容 |
|---|---|
| `swizzle` | GPUスタイルのswizzleアクセサ(`v.xyxz()`など) |
| `unstable` | 0.18.0との互換性のために存在。現時点で到達可能なコードはゲートしていません |
| `serde` | `Serialize`/`Deserialize`の実装 |
| `mint` | [`mint`](https://crates.io/crates/mint) interop型との相互変換 |
| `rand` | ランダム生成用の`Distribution`実装 |

### Swizzling

このライブラリは、GPUプログラマにはお馴染みの
["swizzling"](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics))
というoptional featureを提供します。`--features="swizzle"`で有効化してください。

```rust
let v = Vector3::new(1.0, 2.0, 3.0);
v.xyxz(); // Vector4 { x: 1.0, y: 2.0, z: 1.0, w: 3.0 }
v.zy();   // Vector2 { x: 3.0, y: 2.0 }
```

## 規約

`cgmath-next`はvectorを列(column)ベクトルとして解釈します。つまり、matrixでvectorを変換する際は、matrixが左側に来ます。これは`cgmath-next`が`Matrix * Vector`の乗算演算子は実装していても、`Vector * Matrix`は実装していないことにも表れています。upstreamから変更していません。

## 制限事項

`cgmath-next`はn次元汎用の線形代数ライブラリでは_ありません_。一般的な線形代数よりもコンピュータグラフィックス向けの用途を目的としています。upstreamから変更されていない2・3・4次元の構造体のみを提供します。動的次元のmatrix、GPU compute、`nalgebra`/`glam`スタイルへのAPI再設計は、このリリースシリーズでは明示的にスコープ外です — 全リストは`AGENTS.md`を参照してください。

## セキュリティ報告

[`SECURITY.md`](SECURITY.md)を参照してください。soundnessの問題(safe Rustから到達可能なundefined behavior)は、通常のバグではなくセキュリティ上の問題として扱います。

## 公開状況

**`0.18.1`(stable)はcrates.ioに公開済み**で、タグ付けされています
([`v0.18.1`](https://github.com/kent-tokyo/cgmath-next/releases/tag/v0.18.1))。
その前段階として`0.18.1-alpha.1`のalpha観察期間を経ており、soundnessや互換性に関する重大な問題は報告されませんでした。gatingの経緯全体は[`docs/release-checklist.md`](docs/release-checklist.md)を、このstableシリーズで恒久的な既知制約として受容している唯一の項目については`docs/unsafe-audit.md`の`UNSAFE-002`章を参照してください。

## 貢献方法

[`CONTRIBUTING.md`](CONTRIBUTING.md)を参照してください。

## API差分 / unsafe監査

* [`docs/api-inventory.md`](docs/api-inventory.md) — 0.18.0との機械生成によるpublic API比較
* [`docs/unsafe-audit.md`](docs/unsafe-audit.md) — クレート内の全`unsafe`ブロック、その安全性に関する不変条件、検証状況
