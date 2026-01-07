# Backend RUST Starter

## Instalasi

Unduh dan instal Rust menggunakan rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verifikasi instalasi:

```bash
rustc --version
cargo --version
```

Buat proyek baru:

```bash
cargo new backend-api-jwt
```

## Install Dependensi

Instal `cargo-watch` untuk development:

```bash
cargo install cargo-watch
```

*Jika error: coba aktifkan Copilot :)*

Verifikasi instalasi:

```bash
cargo watch --version
```

Jalankan dengan watch mode:

```bash
cargo watch -q -c -w src/ -x run
```

**Keterangan flags:**
- `-q`: cargo watch dalam mode quiet
- `-c`: clear setiap di-run ulang
- `-w src/`: watch hanya di folder src/
- `-x run`: cargo run setiap ada perubahan

Tambahkan dependencies:

```bash
cargo add axum@0.8.8
cargo add tokio@1.48.0 --features full
cargo add serde@1.0.228 --features derive
cargo add serde_json@1.0.148
cargo add sqlx@0.8.6 --features mysql,runtime-tokio,macros,chrono
cargo add bcrypt@0.17.1
cargo add jsonwebtoken@10.2.0 --features aws_lc_rs
cargo add dotenvy@0.15
cargo add chrono@0.4.42 --features serde
cargo add tower-http@0.6.8 --features cors
cargo add validator@0.20.0 --features derive
cargo install sqlx-cli
```

Periksa kompilasi:

```bash
cargo check
```

## Migrasi DB

Buat migrasi untuk tabel users:

```bash
sqlx migrate add create_users_table
```

Jalankan migrasi:

```bash
sqlx migrate run
```



