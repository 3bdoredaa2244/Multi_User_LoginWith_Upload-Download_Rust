# 📂 FileVault – Multi-User Login with Upload & Download (Rust + ICP)

FileVault is a **decentralized file storage application** built on the [Internet Computer](https://internetcomputer.org/).
It allows multiple users to **authenticate, upload, download, and delete files** securely on-chain.
Files are stored in **chunks**, enabling reliable handling of larger files within ICP canister limits.

---

## ✨ Features

* 🔑 **Multi-user login** via Internet Identity (II).
* 📤 **Upload files** in chunked format (split into smaller pieces).
* 📥 **Download files** by reconstructing chunks.
* 🗑️ **Delete files** securely.
* 📑 **File metadata** (type, size, total chunks) stored alongside content.
* 🌐 **Decentralized backend** powered by Rust canisters.
* 🎨 Simple **frontend UI** for interaction.

---

## 🖼️ Frontend Preview

👉 Insert your screenshot(s) here

Example:

```markdown
![FileVault Frontend](docs/frontend-preview.png)
```

The frontend supports:

* Checking existing files.
* Uploading new files (chunked automatically).
* Downloading stored files.
* Deleting unwanted files.

---

## 🛠️ Tech Stack

* **Rust** – backend canister logic (file storage & chunking).
* **Candid** – interface definition & serialization.
* **IC-CDK** – Internet Computer SDK for Rust.
* **Frontend (React/JS)** – user interface to interact with the canister.
* **DFX** – deployment and local replica management.

---

## ⚙️ Backend API

### Upload a file chunk

```rust
#[update]
fn upload_file_chunk(name: String, chunk: Vec<u8>, index: u64, file_type: String)
```

Uploads a single chunk of a file at the given `index`.

* If the file doesn’t exist, creates metadata.
* Recomputes total file size.

---

### Download a file chunk

```rust
#[query]
fn get_file_chunk(name: String, index: u64) -> Option<Vec<u8>>
```

Fetches the chunk at position `index` for a file.

---

### Check total chunks

```rust
#[query]
fn get_total_chunks(name: String) -> u64
```

Returns how many chunks a file has.

---

### Check if file exists

```rust
#[query]
fn check_file_exists(name: String) -> bool
```

---

### Get file metadata

```rust
#[query]
fn get_file_type(name: String) -> Option<String>
fn get_files() -> Vec<(String, String, u64)>
```

---

### Delete a file

```rust
#[update]
fn delete_file(name: String) -> bool
```

---

## 🚀 Getting Started

### Prerequisites

* Install [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
* Install [Rust](https://www.rust-lang.org/tools/install).
* Node.js & npm (for the frontend).

---

### Local Deployment

1. Clone the repo:

   ```bash
   git clone https://github.com/3bdoredaa2244/Multi_User_LoginWith_Upload-Download_Rust.git
   cd Multi_User_LoginWith_Upload-Download_Rust
   ```

2. Start the IC local replica:

   ```bash
   dfx start --background
   ```

3. Deploy canisters:

   ```bash
   dfx deploy
   ```

4. Start the frontend:

   ```bash
   cd frontend
   npm install
   npm start
   ```

5. Open the app at:

   ```
   http://localhost:3000
   ```

---

## ⚠️ Limitations

* No **compression** or **encryption** yet (raw chunk storage).
* Files larger than a few MB may still require optimizations.
* Not audited for production security.

---

## 📌 Future Improvements

* ✅ Add **file encryption** for privacy.
* ✅ Integrate **IPFS or hybrid storage** for very large files.
* ✅ Implement **progress bars** & resumable uploads.
* ✅ Add **sharing & permissions** between users.

---

## 👨‍💻 Authors

* **AbdulRahmann Redaa** – Software Engineer, Web3 & Blockchain Developer
