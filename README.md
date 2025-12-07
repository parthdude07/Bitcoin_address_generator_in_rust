# 🦀 Bitcoin Address Generator (From Scratch)

A minimalist command-line tool written in **Rust** that generates a Bitcoin Private Key and derives its corresponding Legacy Address (P2PKH). 

**Why this exists:** This project was built as a study exercise. Instead of using the standard `rust-bitcoin` library, I implemented the protocol logic manually using primitive cryptographic libraries (`k256`, `sha2`, `ripemd`) to deeply understand the mathematical pipeline of Bitcoin identities.

---

## 🚀 Features

- **Entropy Generation:** Uses OS-level randomness (`OsRng`) to generate secure 256-bit secrets.
- **Elliptic Curve Math:** Manually derives Compressed Public Keys using the **secp256k1** curve.
- **Hashing Pipeline:** Implements the `Hash160` standard (SHA-256 followed by RIPEMD-160).
- **Checksum Validation:** Calculates the double-SHA256 checksum to prevent typing errors.
- **Base58 Encoding:** Converts the raw byte payload into the standard Bitcoin address format.

---

## 🛠️ Tech Stack

- **Language:** Rust 🦀
- **Elliptic Curves:** `k256` (Pure Rust implementation of secp256k1)
- **Hashing:** `sha2`, `ripemd`
- **Encoding:** `bs58`, `hex`

---

## 📚 How It Works (The Pipeline)

The code follows the standard Bitcoin address generation steps:

1.  **Generate Private Key:** $k$ (256-bit integer)
2.  **Derive Public Key:** $K = k \times G$ (Elliptic Curve Multiplication)
3.  **Hash160:** $H = \text{RIPEMD160}(\text{SHA256}(K))$
4.  **Add Version Byte:** Prepend `0x00` (Mainnet)
5.  **Checksum:** Take first 4 bytes of $\text{SHA256}(\text{SHA256}(\text{Version} + H))$
6.  **Encode:** Base58 encode the result.

---


