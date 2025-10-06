# sitewatcheR

**sitewatcheR** is a Rust remake of an old Go project. This program monitors the HTML content of a specified website and notifies you whenever changes are detected. It can also log these changes to a file.

---

## ⚡ Features

* Continuously monitors a given URL.
* Prints a message to the console when HTML content changes.
* Optionally logs changes to `log.txt`.
* Optionally prints "No change found" messages even when the content hasn't changed.

---

## 🛠 Requirements

* Rust (stable)
* `reqwest` crate
* `chrono` crate

Cargo.toml example:

```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking"] }
chrono = "0.4"
```

---

## 🚀 Installation & Running

1. Clone the repository or download the files:

```bash
git clone https://github.com/<your_username>/sitewatcheR.git
cd sitewatcheR
```

2. Build the project:

```bash
cargo build --release
```

3. Run the program:

```bash
cargo run
```

4. The program will prompt you for the following information:

```
Enter URL to watch : [Website URL to monitor]
Enter time to wait between requests (seconds) : [Interval in seconds]
Do you want to get notified when cannot find? (y for yes, anything for no) :
Do you want log? (y for yes, anything for no) :
```

---

## 💾 Log File

* The `log.txt` file is used to store change logs.
* If logging is enabled, any detected changes are written to the file automatically.
* If the file does not exist, it will be created; if it exists, new entries are appended.

---

## ⚙️ Example Usage

```
Enter URL to watch : https://example.com
Enter time to wait between requests (seconds) : 5
Do you want to get notified when cannot find? (y for yes, anything for no) : y
Do you want log? (y for yes, anything for no) : y
```

The program will check the site every 5 seconds and log changes both to the console and the log file.

---

## 📌 Notes

* `reqwest` works with HTTP/HTTPS URLs. `file://` URLs are not supported.
* Ensure `log.txt` is added to `.gitignore` if you don't want it in the repository.

---