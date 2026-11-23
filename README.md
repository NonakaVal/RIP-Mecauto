# MecAuto

Sistema de gestão para mecânica automotiva, desenvolvido em Rust.

## Requisitos

- [Rust](https://www.rust-lang.org/) (edition 2021) — instale via [rustup](https://rustup.rs)
- Um compilador C (`gcc`/`clang`) — necessário para compilar o SQLite embutido do `rusqlite` (feature `bundled`)
- Bibliotecas de sistema exigidas pelo `eframe`/`egui` (X11, OpenGL e Wayland no Linux)

## Dependências

As dependências são declaradas no [`Cargo.toml`](Cargo.toml):

| Crate        | Versão                      | Uso                          |
| ------------ | --------------------------- | ---------------------------- |
| eframe, egui | 0.37                        | Interface gráfica            |
| rusqlite     | 0.32 (`bundled`, `backup`)  | Banco de dados SQLite        |
| serde        | 1 (`derive`)                | Serialização de dados        |
| serde_json   | 1                           | Leitura/escrita de JSON      |
| image        | 0.25                        | Carregamento de imagens      |
| zip          | 2 (`deflate`)               | Backups compactados          |

> O `cargo build` baixa e compila automaticamente todas as crates e as fixa no [`Cargo.lock`](Cargo.lock). Não há um instalador separado de dependências.

## Instalação

### 1. Instalar o Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Instalar as dependências de sistema

**Ubuntu / Debian:**

```bash
sudo apt update
sudo apt install build-essential pkg-config libx11-dev libxi-dev libxkbcommon-dev libgl1-mesa-dev
```

**Arch Linux e derivados (CachyOS, Manjaro, EndeavourOS):**

```bash
sudo pacman -Syu --needed base-devel pkg-config libx11 libxcursor libxrandr libxi libxkbcommon mesa wayland
```

Para instalar o Rust nesses sistemas:

```bash
sudo pacman -S --needed rustup
rustup default stable
```

### 3. Baixar e compilar as dependências

```bash
cargo build --release
```

## Executar

```bash
cargo run --release
```

## Interface e estoque

A interface usa navegação lateral, cartões de resumo e superfícies claras com azul
para ações principais. Em **Produtos**, pesquise por descrição, código, fabricante
ou código de barras e use **Dar saída** no item para registrar a quantidade e uma
observação opcional. O estoque é atualizado e a movimentação fica disponível no
dashboard; registros são preservados no banco SQLite e nos backups portáteis.
