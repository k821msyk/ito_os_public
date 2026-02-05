# ITO-OS: Quantum-Generic Core (v0.1.0)

> **"What if your desktop came with a localized quantum processing unit?"** 
> 家庭用PCに量子チップが載る時代のOSはどうあるべきか、という空想の記録です。

## 🌌 Overview / 概要
A snapshot of an experimental bare-metal kernel written in Rust for x86_64.  
This project is a personal thought experiment on managing "Entropy (Instability)" as a critical system resource.

x86_64環境で動作する、Rust製の自作OS試作コード断片です。  
物理的な「不安定性（エントロピー）」をシステムのリソース管理に組み込む実験を、Rust nightlyを用いて行いました。

**⚠️ Note / 補足**
- Personal snapshot of my learning process. This version (v0.1.0) marks the first public record.  
  学習と試行錯誤の過程をそのまま残した、個人的な記録です。
- This is a simulation and does NOT control real quantum hardware.  
  実在の量子ハードを制御するものではありません。

## 🧪 Implemented Elements / 実装済み要素
- **QRM (Quantum Resource Management)**: Exclusive slot management using Rust's ownership model.  
- **Entropy Monitoring**: Visualization of system instability and collapse (Panic) mechanics.  
- **VGA Direct Rendering**: Real-time status display via VGA text mode.  

## 🕹️ How to Run / 動作確認
Requires QEMU.

```bash
cargo install bootimage
cargo run
```

- `A`: Allocate virtual Qubit / 確保
- `R`: Reset all systems / 全リセット
- `SPACE`: Interfere field (Increase entropy) / フィールド干渉

---
© ITO-OS Project. Licensed under the MIT License.