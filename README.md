# ITO-OS: Quantum-Generic Core (v0.2.1)

> **"What if your desktop came with a localized quantum processing unit?"**

<details>
<summary>Japanese Translation / 日本語訳</summary>

「家庭用PCに量子チップが載る時代のOSはどうあるべきか、という空想の記録。」
</details>

## 🌌 Overview
A bare-metal kernel written in Rust for x86_64, featuring a mathematical Quantum State Simulation engine. v0.2.1 shifts from simple noise simulation to tracking complex probability amplitudes and wave-function collapse.

<details>
<summary>Japanese Translation / 日本語訳</summary>

x86_64環境で動作する、Rust製の自作OSカーネル。v0.2.1では、単なるノイズシミュレーションを超え、複素振幅の保持と測定による「波動関数の収縮」を数学的に実装しています。
</details>

## 🤖 About Development
This project was designed and implemented through collaboration with AI assistants (Claude / Google Gemini) as a learning exercise in quantum computing concepts and OS development.

<details>
<summary>Japanese Translation / 日本語訳</summary>

このプロジェクトは、AIアシスタント（Claude / Google Gemini）との対話を通じて設計・実装された学習プロジェクトです。量子コンピューティングの基本概念と、Rustによるベアメタル開発の理解を深めることを目的としています。
</details>

**⚠️ Note**
- **Mathematical Evolution**: Introduces a complex number class using 16-bit fixed-point arithmetic to manage state vectors: $|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$.
- **Hardware Agnostic**: A thought-experiment project simulating fundamental quantum laws at the kernel level.

<details>
<summary>Japanese Translation / 日本語訳</summary>

- **数学的な進化**: 16-bit固定小数点演算による複素数クラスを導入。$|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$ の状態ベクトルを管理します。
- **ハードウェア非依存**: 物理法則（量子力学の基礎）をカーネルレベルでシミュレートする思考実験的プロジェクトです。
</details>

## 🧪 Key Features (v0.2.1)
- **Quantum State Vector Engine**: Maintains complex amplitudes and generates superposition states using Hadamard transforms.
- **Probabilistic Measurement**: Implements wave-function collapse based on the square of the amplitudes (Born rule).
- **Real-time State Visualization**: Dynamic display of quantum states (SUPER / |0> / |1>) and probability gauges per unit.

<details>
<summary>Japanese Translation / 日本語訳</summary>

- **量子状態ベクトルエンジン**: 複素振幅の保持と、アダマール変換による「重ね合わせ状態」の生成。
- **確率的測定**: 振幅の二乗に基づいた確率的な状態確定（波動関数の収縮）ロジックの実装。
- **リアルタイム可視化**: ユニットごとの量子状態（SUPER / |0> / |1>）と確率ゲージの動的表示。
</details>

## 🕹️ How to Run
Requires QEMU and Rust Nightly (2026-01-20 or later).

```bash
# Install required components
rustup component add llvm-tools-preview
cargo install bootimage

# Build and Run
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ito_os/debug/bootimage-ito_os.bin
```

### Controls
- **[A]**: **Allocate + H** / Initialize a unit with Hadamard state $|+\rangle$. (ユニットを確保しアダマール状態で初期化)
- **[R]**: **Reset** / Full system reset and release all units. (システムを全リセットし全ユニットを解放)
- **[SPACE]**: **Measure** / Measure the active unit and collapse its state. (ユニットを測定し状態を確定させる)

---
© 2026 ITO-OS Project. Licensed under the MIT License.