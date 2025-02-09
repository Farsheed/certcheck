# Certificate Checker in Rust

This Rust application checks the expiration date of TLS certificates for given URLs. It reads URLs from a file, connects to each via TLS, and provides a color-coded output based on certificate validity.

## Features
- Reads a list of URLs from a file (`urls.txt`).
- Connects securely to retrieve certificate expiration dates.
- Outputs results in a color-coded format:
  - 🟥 **Red**: Certificate expired.
  - 🟧 **Orange**: Expires within 24 hours.
  - 🟨 **Yellow**: Expires within one week.
  - 🟩 **Green**: Certificate valid.

## Requirements
- [Rust & Cargo](https://rustup.rs) installed.
- OpenSSL installed (`brew install openssl` on macOS if needed).

## Installation & Setup

### 1. Clone project
```bash
git clone https://github.com/Farsheed/certcheck.git
cd certcheck
```
### 2. Add URLs to the urls.txt file
In the project root, add the URLs you want to check to `urls.txt` one URL per line.
You can omit the protocol (https) from the URL.

### 5. Run the Project
```bash
cargo run
```

## Notes
- Ensure OpenSSL is properly linked if you encounter build issues.
- This application assumes all timestamps are in UTC/GMT.
