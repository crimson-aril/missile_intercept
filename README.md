# Missile Intercept

A 2D missile interception game made with **Godot 4** and **Rust** using [godot-rust/gdext](https://github.com/godot-rust/gdext).

## Gameplay

![Gameplay](assets/gameplay.gif)

---

## Requirements

* [Godot 4](https://godotengine.org/)
* [Rust](https://rust-lang.org/tools/install/)
* [Git](https://git-scm.com/)
* C/C++ compiler and linker

---

## 1. Install Rust

Rust requires a C/C++ compiler and linker for building Rust projects.

### Windows

Install **Visual Studio** or **Visual Studio Build Tools**:

https://visualstudio.microsoft.com/downloads/

During installation, select:

**Desktop development with C++**

Make sure the required **MSVC C++ build tools** and **Windows SDK** are installed.

After installing the C/C++ tools, install Rust:

https://rust-lang.org/tools/install/

Verify the installation:

```powershell
rustc --version
cargo --version
```

### Linux

Install the required C/C++ build tools.

For Ubuntu/Debian:

```bash
sudo apt update
sudo apt install build-essential
```

Then install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Activate your environment by running

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### macOS

Install Apple's Command Line Tools:

```bash
xcode-select --install
```

Then install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rust.rs | sh
```

Verify the installation:

```bash
rustc --version
cargo --version
```

---

# 2. Download gdext

This project was created using a **manually downloaded copy of `godot-rust/gdext`**.

Download the `gdext` repository from:

https://github.com/godot-rust/gdext

Make sure you use the **`master` branch**.

---

# 3. Place `gdext-master` and the Project Together

The `gdext-master` folder and the **Missile Intercept** project must be located in the same parent directory.

For example:

```text
Projects/
├── gdext-master/
│   ├── godot/
│   ├── godot-core/
│   ├── godot-ffi/
│   └── ...
│
└── missile Intercept/
    ├── assets/
    ├── scenes/
    ├── scripts/
    ├── src/
    ├── Cargo.toml
    ├── missile-defense.gdextension
    ├── project.godot
    └── README.md
```

The important part is that:

```text
missile Intercept/
```

and:

```text
gdext-master/
```

are **siblings**.

Do not place `gdext-master` inside the Missile Intercept project folder.

---

# 4. Configure `Cargo.toml`

Because `gdext-master` is located next to the project, the project uses a local path dependency.

In:

```text
missile Intercept/Cargo.toml
```

use:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
godot = { path = "../gdext-master/godot" }
```

The important part is:

```text
../gdext-master/godot
```

This path goes:

```text
missile Intercept/
        ↓
..      → parent directory
        ↓
gdext-master/
        ↓
godot/
```

### Important

If you move either `gdext-master` or the Missile Intercept project, the relative path may no longer work.

Keep them in the same parent directory unless you update the path in `Cargo.toml`.

---

# 5. Alternative: Download gdext Automatically with Cargo

You can also use `gdext` directly from Git instead of manually downloading it.

Replace the local path dependency:

```toml
[dependencies]
godot = { path = "../gdext-master/godot" }
```

with:

```toml
[dependencies]
godot = { git = "https://github.com/godot-rust/gdext", branch = "master" }
```

With this method, you **do not need to manually download `gdext-master`**.

Cargo will download the `godot-rust/gdext` dependency from the `master` branch when building the project.

---

# 6. Build the Project

Open a terminal in the **Missile Intercept project directory**:

```bash
cd "missile Intercept"
```

Then build the Rust extension:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

---

# 7. Open the Project in Godot

Open **Godot 4** and import:

```text
missile Intercept/project.godot
```

The project uses:

```text
missile-defense.gdextension
```

to load the compiled Rust GDExtension.

---

# 8. Run the Game

After successfully building the Rust project, run the game from Godot.

* **F5** — Run Project
* **F6** — Run Current Scene

---

# 9. Rebuild After Rust Changes

Whenever you modify the Rust source code, run:

```bash
cargo build
```

Then run the project again in Godot.

---

## Documentation

* [Folder Structure](FOLDER_STRUCTURE.md)
* [Credits & Asset Acknowledgements](CREDITS.md)
* [License](LICENSE.md)

## References

* [godot-rust/gdext](https://github.com/godot-rust/gdext)
* [Rust Installation](https://rust-lang.org/tools/install/)
* [Godot Engine](https://godotengine.org/)
