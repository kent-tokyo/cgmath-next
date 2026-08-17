# cgmath-next

[![CI](https://img.shields.io/github/actions/workflow/status/kent-tokyo/cgmath-next/ci.yml?branch=main&label=CI)](https://github.com/kent-tokyo/cgmath-next/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/docsrs/cgmath-next)](https://docs.rs/cgmath-next)
[![crates.io](https://img.shields.io/crates/v/cgmath-next.svg)](https://crates.io/crates/cgmath-next)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/kent-tokyo/cgmath-next/blob/main/LICENSE)

[English](README.md) | [日本語](README_ja.md) | 中文

**还在使用 `cgmath` 0.18 吗?** `cgmath-next` 是它持续维护、源码兼容的后继版本 ——
API 完全相同,[已知的 soundness 问题](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)已修复。

> 保留 API,消除 unsoundness(健全性缺陷)。

## 迁移方法

大多数情况下,迁移只需要修改 `Cargo.toml` 中的一行:

```toml
[dependencies]
cgmath = { package = "cgmath-next", version = "0.18.1" }
```

```rust
use cgmath::{Matrix4, Quaternion, Vector3}; // 无需修改
```

这是因为 `cgmath-next` 编译后的库名仍然是 `cgmath`(基于 fixture 的验证详见 [`docs/compatibility.md`](docs/compatibility.md))。更多细节,以及甚至无需改名的情况,请参见 [`docs/migration.md`](docs/migration.md)。

## 为什么要迁移

| | `cgmath` 0.18 | `cgmath-next` 0.18.1 |
|---|---|---|
| API | — | 相同 |
| 维护状态 | [已无人维护](https://rustsec.org/advisories/RUSTSEC-2026-0196.html) | 持续积极维护 |
| 已知的 swap UB([RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)) | 受影响 | 已修复 |
| import 改动 | — | 无需 |
| public API 差异 | — | 零 |
| 许可证 | Apache-2.0 | Apache-2.0 |

`cgmath-next` 是基于 [`cgmath`](https://github.com/rustgd/cgmath) 0.18.0 独立开发和维护的社区后继库 —— 并非官方延续项目。

## 示例

一个最小可运行示例:构建相机的 view-projection 矩阵,并将一个点变换到
clip space 和 normalized device coordinates —— 这与渲染器针对每个顶点
执行的流程相同。详见
[`examples/camera_transform.rs`](examples/camera_transform.rs)。

```
cargo run --example camera_transform
```

## 本项目保留的内容

* `cgmath` 0.18.0(crates.io 发布版)的 public API — 已通过机器生成的路径差异对比验证,目前**零差异**(详见 [`docs/api-inventory.md`](docs/api-inventory.md))
* 数值计算结果,除 soundness 修复所必需的改动外均保持不变 — 已通过针对真实 `cgmath` 0.18.0 的 9 项 differential test 验证,采用精确(而非近似)相等性比较(详见 [`docs/compatibility.md`](docs/compatibility.md))
* 类型与 trait 实现,包括 `serde`/`mint`/`rand` 的 derive 实现 — 代码本身未作修改,但其线上格式(wire-format)输出尚未针对 upstream 进行独立的往返(round-trip)测试(同样记录于 `docs/compatibility.md`)
* 原始 crate 已声明的 `[lib] name = "cgmath"` — 现有的 `use cgmath::...` 代码可以直接编译通过
* Apache-2.0 许可证与原始版权声明

## 本项目不保证的内容

* **不代表"100% 内存安全"或"已完全审计"的声明。** Soundness 相关工作仍在持续进行,并在 [`docs/unsafe-audit.md`](docs/unsafe-audit.md) 中逐项跟踪。现有 unsafe 代码中的一类(`AsRef`/`AsMut`/`From` 到同类型 tuple 的转换,例如 `(f32, f32, f32)`)依赖于 Rust 语言参考并未正式保证的 tuple 内存布局 — 记录为 `UNSAFE-002`。目前已对其进行了**加固(guarded)**处理:每次此类转换前都会在运行时校验大小、对齐方式以及各字段的字节偏移量,如果不匹配则 panic 而不是 transmute(已通过 Miri、negative-control 测试,以及确认在布局匹配的当前情况下零开销的 release build 反汇编进行验证)。这将未来潜在的布局差异从静默的未定义行为(undefined behavior)转变为立即可察觉的 panic — 但这**并不是语言层面的 soundness 证明**;tuple 布局在官方层面仍未被规定,唯一能彻底解决这个问题的方法是移除返回引用的转换(属于 public API 变更,超出本系列的范围)。完整说明请参见 `docs/unsafe-audit.md` 的可行性研究章节,以及独立佐证本问题并非本 fork 特有的 [`rustgd/cgmath#538`](https://github.com/rustgd/cgmath/issues/538)。
* **不保证 Rust ABI 兼容性。** `#[repr(C)]` 布局(`size_of`、`align_of`、字段偏移量)仅针对声明了该属性的类型进行验证,并不适用于整个 crate。
* **并非适用于所有场景的"完全可直接替换方案"。** 已验证的兼容性范围仅限于实际测试过的内容:未经修改且全部通过的 upstream 测试套件、若干依赖改名(dependency-rename)fixture,以及 5 个真实的下游 crate(详见 [`compat/fixtures/reverse-deps/RESULTS.md`](compat/fixtures/reverse-deps/RESULTS.md))。具体验证了哪些内容,请参见 [`docs/compatibility.md`](docs/compatibility.md)。

## RustSec 通告状态

| 通告 | 状态 |
|---|---|
| [RUSTSEC-2026-0197](https://rustsec.org/advisories/RUSTSEC-2026-0197.html)(soundness:`swap_columns` 相同索引 UB) | **已修复**,且修复范围超出通告字面描述 — 详见 [`docs/unsafe-audit.md`](docs/unsafe-audit.md) 以及项目提交历史中关于 `Array::swap_elements` 和 `Matrix::swap_elements` 的部分,这两处存在相同的 bug 但通告中并未提及。 |
| [RUSTSEC-2026-0196](https://rustsec.org/advisories/RUSTSEC-2026-0196.html)(无人维护) | 本项目的存在本身即是应对措施:`cgmath-next` 目前正在积极维护。 |

## 与 upstream 的关系

`cgmath-next` 基于在 crates.io 上发布的 `cgmath` 0.18.0,而非 upstream 的 `master` 分支(该分支已因未发布的功能和依赖版本升级而产生分歧)。关于确切来源、校验和,以及自 0.18.0 标签以来每个 upstream commit 的分类,详见 [`docs/provenance.md`](docs/provenance.md)。目前已发布的内容请参见[发布状态](#发布状态)。

## 兼容性政策

* Public API 的删除、重命名、签名变更或 trait bound 收紧,在 0.18.1 系列中均视为发布阻断项(release blocker),不会被悄悄引入。
* 当 soundness 修复与严格的行为保持发生冲突时,优先选择 soundness 修复(例如:相同索引的 swap 变为显式的 no-op)。
* 新功能和大规模重新设计将推迟到 API 对等版本发布之后。本项目遵循的完整政策详见本仓库中的 `AGENTS.md`。

## MSRV

关于实测的最低支持 Rust 版本及其测定方法,详见 [`docs/msrv.md`](docs/msrv.md)。

## Features(功能特性)

显式声明与隐式生成的 feature 的区别(Cargo 会为每个 optional dependency 自动生成一个 feature)— 完整说明详见 [`docs/compatibility.md`](docs/compatibility.md):

| Feature | 作用 |
|---|---|
| `swizzle` | GPU 风格的 swizzle 访问器(如 `v.xyxz()`) |
| `unstable` | 为与 0.18.0 保持兼容而保留;目前不会启用任何可达代码 |
| `serde` | `Serialize`/`Deserialize` 实现 |
| `mint` | 与 [`mint`](https://crates.io/crates/mint) 互操作类型之间的相互转换 |
| `rand` | 用于随机数生成的 `Distribution` 实现 |

### Swizzling

本库提供了一个名为
["swizzling"](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics))
的可选 feature,GPU 程序员对此应该都很熟悉。使用 `--features="swizzle"` 启用它。

```rust
let v = Vector3::new(1.0, 2.0, 3.0);
v.xyxz(); // Vector4 { x: 1.0, y: 2.0, z: 1.0, w: 3.0 }
v.zy();   // Vector2 { x: 3.0, y: 2.0 }
```

## 约定

`cgmath-next` 将 vector 解释为列向量(column vector),也就是说,用 matrix 变换 vector 时,matrix 位于左侧。这体现在 `cgmath-next` 实现了 `Matrix * Vector` 的乘法运算符,但没有实现 `Vector * Matrix`。这一点与 upstream 保持一致,未作改动。

## 限制

`cgmath-next` _并非_ 一个 n 维通用线性代数库,其目标是面向计算机图形学应用,而非通用线性代数。它只提供 2、3、4 维的结构体,这些都是从 upstream 原样继承而来。动态维度的 matrix、GPU compute,以及类似 `nalgebra`/`glam` 风格的 API 重新设计,均明确排除在本发布系列的范围之外 — 完整列表详见 `AGENTS.md`。

## 安全问题报告

详见 [`SECURITY.md`](SECURITY.md)。Soundness 问题(即 safe Rust 可达的未定义行为)将被视为安全问题处理,而非普通 bug。

## 发布状态

**`0.18.1`(stable)已发布**到 crates.io,并已打标签
([`v0.18.1`](https://github.com/kent-tokyo/cgmath-next/releases/tag/v0.18.1))。
在此之前经过了 `0.18.1-alpha.1` 的 alpha 观察期,期间未收到任何 soundness
或兼容性方面的重大问题报告。完整的发布条件历史详见
[`docs/release-checklist.md`](docs/release-checklist.md);本 stable
系列中唯一被接受为永久性已知限制的事项,详见 `docs/unsafe-audit.md` 中的
`UNSAFE-002` 章节。

## 贡献指南

详见 [`CONTRIBUTING.md`](CONTRIBUTING.md)。

## API 差异 / unsafe 审计

* [`docs/api-inventory.md`](docs/api-inventory.md) — 与 0.18.0 之间的机器生成 public API 对比
* [`docs/unsafe-audit.md`](docs/unsafe-audit.md) — crate 中每一个 `unsafe` 代码块及其安全性不变式与验证状态
