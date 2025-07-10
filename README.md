# 🔥 HashLaser – Intelligent File Deduplicator

> **Hash it. Slash it. Save space like a pro.**

`HashLaser` is a blazing-fast, intelligent CLI tool written in Rust to scan directories, detect duplicate files using SHA-256, filter them based on criteria, and optionally delete them. Designed for developers and power users who want full control over their filesystem hygiene.

---

## 🚀 Features

- ✅ Compare two files using SHA-256 hash
- 📁 Scan entire directories for duplicate files
- 🔍 Filter by file size, extension, name pattern
- 🧪 Dry-run deletion mode
- 🧾 Generate JSON reports of duplicate groups
- 🌈 Clean CLI output (banner, emojis, colors)
- 🧪 Modular with unit tests
- 📚 RustDocs documentation

---

## 📦 Installation

```bash
git clone https://github.com/your-username/hashlaser.git
cd hashlaser
cargo build --release
```

> Requires: Rust & Cargo installed → [https://rustup.rs](https://rustup.rs)

---

## ⚙️ How to Run

All commands follow this structure:

```bash
cargo run -- <command> [args/options]
```

---

## 💡 Usage Examples

### 📊 1. Compare Two Files

```bash
cargo run -- compare path/to/file1.txt path/to/file2.txt
```

### 🗂️ 2. Scan a Directory for Duplicates

```bash
cargo run -- scan assets
```

With filters:

```bash
cargo run -- scan assets --min 100 --max 10000 --ext txt,csv --regex ".*report.*"
```

### 🔎 3. Filter Files (show without deleting)

```bash
cargo run -- filter assets --ext jpg,png --min 5000
```

### 🧾 4. Generate JSON Report

```bash
cargo run -- report assets output.json
```

### 🧪 5. Delete Duplicate Files (Dry Run)

```bash
cargo run -- delete assets --dry-run
```

### 🗑️ 6. Actually Delete Duplicates

```bash
cargo run -- delete assets
```

---

## ✅ Test Coverage

To run all test cases:

```bash
cargo test
```

> Unit tests available for hashing, scanning, filtering, and safe deletion.

---

## 📚 Documentation

To generate and view Rust Docs:

```bash
cargo doc --no-deps --open
```

Docs are generated in:

```
target/doc/hashlaser/index.html
```

---

## 👨‍💻 Contributing

Follow these steps to contribute:

```bash
# 1. Fork the repo on GitHub
git clone https://github.com/your-username/hashlaser.git
cd hashlaser

# 2. Add upstream (original)
git remote add upstream https://github.com/original/hashlaser.git

# 3. Create a new branch
git checkout -b fix-typo-readme

# 4. Make changes
git add .
git commit -m "Fix typo in README"

# 5. Push and create PR
git push origin fix-typo-readme
gh pr create --base main --head your-username:fix-typo-readme --title "Fix typo" --body "Fixed typo in README"
```

---

## 🏷️ Tags

`Rust` · `SHA-256` · `Filesystem` · `Duplicate Files` · `CLI` · `Deduplication`

---

## 👤 Author

**Debansu Debadutta Das**  
🔗 [My Portfolio](https://sites.google.com/view/debansu-debadutta-das/home)  
📫 Email: debansudebaduttadas@gmail.com

---

## ⭐ Star if you like it!

If `HashLaser` helped you save disk space or learn Rust, give it a ⭐ on GitHub!

---