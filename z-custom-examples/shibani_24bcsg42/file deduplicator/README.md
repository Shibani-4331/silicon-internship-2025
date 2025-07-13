📁 Advanced File Deduplicator in Rust

A powerful Rust-based command-line tool to identify and delete duplicate files in any folder (even outside current directory) only in D-drive, using multiple hashing algorithms and generating a detailed JSON report.


---

🚀 Features

✅ Detect duplicate files using SHA-256, Blake3, and XxHash64

📁 Accepts just the folder name and locates it anywhere (within 5 levels of current directory)

🔁 Recursively scans all subfolders

⚙ Filters by file size and type (customizable)

📄 Outputs a duplicate_report.json file

🗑 Optionally deletes duplicates while keeping one copy



---

🛠 Setup Instructions

1. 📦 Prerequisites

Install Rust

Use a terminal or VS Code


2. 📁 Folder Structure

project_root/
├── src/
│   └── main.rs  ← Rust code
├── Cargo.toml   ← dependencies

3. 🧰 Add These Dependencies to Cargo.toml

[dependencies]
rayon = "1.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sha2 = "0.10"
blake3 = "1.5"
twox-hash = "1.6"
walkdir = "2.4"


---

▶ How to Run

cargo run

Then enter just the folder name when prompted (no need for full path):

📁 Enter folder name :
MyFolder

If duplicates are found, a JSON file will be created. You will be asked if you want to delete the extra copies.


---

🔄 Flowchart

flowchart TD
    A[User enters folder name] --> B[Search recursively for folder up to 5 levels]
    B --> C[Check each file extension and size filter]
    C --> D[Choose hash algorithm based on file type]
    D --> E[Generate file hash and group by hash]
    E --> F[If hash appears more than once -> duplicate group]
    F --> G[Generate JSON report]
    G --> H{User chooses to delete?}
    H -- Yes --> I[Delete all but one in each group]
    H -- No --> J[Exit safely]


---

📂 Output Example (duplicate_report.json)

[
  {
    "hash": "a1b2c3d4...",
    "files": [
      "path/to/file1.txt",
      "path/to/copy_of_file1.txt"
    ]
  }
]

---

👩‍💻 Author

Shibani Mishra

> Feel free to contribute, fork, or open issues!