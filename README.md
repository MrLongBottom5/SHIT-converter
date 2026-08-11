# 💩 SHIT Converter

**SHIT** — *Some Highly Interesting Text* — is a tiny custom binary text format and converter written in **Rust**.

Yes, it makes `.shit` files.

No, it is not efficient.

In fact, it's approximately **380% more bloated than a normal `.txt` file**.

That's a feature. 

---

## 🚀 What can it do?

SHIT currently supports:

* 📝 Encoding text into `.shit`
* 📖 Decoding `.shit` back into text
* 🌍 UTF-8 characters
* ↵ Newlines
* **Bold text**
* *Italic text*
* 🗃️ A custom `SHIT1` file header
* ⚙️ Custom binary encoding
* 🦀 Written entirely in Rust

---

## 📦 SHIT Commands

SHIT has its own tiny markup system.

| Command | Meaning      |
| ------- | ------------ |
| `\n`    | New line     |
| `\b`    | Start bold   |
| `b/`    | Stop bold    |
| `\i`    | Start italic |
| `i/`    | Stop italic  |

### Example

```text
Hello \bthis is boldb/!

This is \ithis is italic i/.

This is a new line:\nHere!
```

When decoded in a terminal supporting ANSI formatting, the formatting commands are rendered appropriately.

---

## 🛠️ Building

You'll need:

* Rust
* Cargo
* Git

Clone the repository:

```bash
git clone https://github.com/MrLongBottom5/SHIT-converter.git
```

Enter the directory:

```bash
cd SHIT-converter
```

Build it:

```bash
cargo build
```

Or build an optimized release version:

```bash
cargo build --release
```

---

## ▶️ Running

Run directly with Cargo:

```bash
cargo run
```

Or run the compiled release version:

```bash
./target/release/shit-converter
```

---

## 🧑‍💻 Using SHIT

When the program starts, you'll see:

```text
-----SHIT-CONVERTER-AND-READER-----
Do you want to encode text to shit or decode shit to text (d/e) or make a new file
```

### Encode an existing text file

Enter:

```text
e
```

Then enter your text file:

```text
text.txt
```

Then choose the name of your SHIT file:

```text
myshit
```

The program creates:

```text
myshit.shit
```

---

### Decode a SHIT file

Enter:

```text
d
```

Then:

```text
myshit.shit
```

The contents will be decoded and printed to the terminal.

---

### Create a SHIT file from the terminal

Enter:

```text
n
```

Then provide a filename and the text you want to encode.

---

## 📊 SHIT vs TXT

| Format  |                   Size |
| ------- | ---------------------: |
| `.txt`  |               🟢 Small |
| `.shit` | 🔴 Why is this so big? |

Compression:

```text
TXT  █
SHIT █████
```

**SHIT is not designed to save disk space.**

Your SSD has been warned.

---


## ⚠️ Disclaimer

This project is completely serious.

It is also completely stupid.

It exists because making a custom binary file format sounded fun.

---

## 🦀 Why Rust?

Because I was learning Rust and thought:

> "Why not make a file format?"

And somehow this happened.

---

## 📜 License

Do whatever you want with this project.

Have fun.

Make more shit. 💩
