# ITO-OS: Quantum-Generic Core (v0.2.0)

> **"What if your desktop came with a localized quantum processing unit?"**

> 家庭用PCに量子チップが載る時代のOSはどうあるべきか、という空想の記録。

## 🌌 Overview / 概要
A bare-metal kernel written in Rust for x86_64, featuring a "High-Fidelity" quantum simulation engine.  
v0.2.0 focuses on the interaction between system stability and resource allocation.

x86_64環境で動作する、Rust製の自作OSカーネル。  
v0.2.0では「高再現度シミュレーション」をテーマに、システム全体の安定性とリソース管理の相互作用を実装しています。

**⚠️ Note / 補足**
- **Snapshot of Evolution**: v0.1.0の試作を経て、メモリ衝突や描画ロジックの課題を解決した安定版です。
- **Hardware Agnostic**: 物理法則の振る舞い（エントロピー拡散・デコヒーレンス）をカーネルレベルでシミュレートしています。

## 🧪 Key Features (v0.2.0) / 実装済み要素
- **HF-QPS (High-Fidelity Quantum Physics Simulation)**: 
  - 疑似乱数によるノイズ伝搬アルゴリズムの実装。
  - エントロピー増大に伴うビット反転と、自動冷却（Cooling）ロジックの導入。
- **QRM (Quantum Resource Management)**: 
  - 8つの物理スロットの排他的管理。
  - コヒーレンス（量子コヒーレンス）のリアルタイム減衰計算。
- **Advanced VGA Display**: 
  - バックバッファを用いたティアリングのないステータス描画。
  - エントロピーレベルに応じたビジュアル・フィードバック。

## 🕹️ How to Run / 動作確認
Requires QEMU and Rust Nightly (2026-01-20 or later).

```bash
# 必要コンポーネントの導入
rustup component add llvm-tools-preview
cargo install bootimage

# 構築と実行
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ito_os/debug/bootimage-ito_os.bin
```

### Controls / 操作方法
- **[A]**: **Allocate** / 仮想ユニットを起動し、リソースを占有する。
- **[R]**: **Reset** / システムを全初期化し、エントロピーをリセットする。
- **[SPACE]**: **Interfere** / フィールドに意図的な干渉を与え、カオスを誘発する。

---
© 2026 ITO-OS Project. Licensed under the MIT License.