# Hack Assembler | Systems Logic & Binary Translation

A robust two-pass assembler written in Rust, designed to translate symbolic Hack Assembly (.asm) into 16-bit binary machine code. This project demonstrates foundational knowledge of deterministic translation, symbol table management, and bit-level instruction encoding.

### 🛠 Technical Highlights
* **Two-Pass Translation Logic:** Engineered a two-pass architecture to handle forward-referencing labels. The first pass populates the Symbol Table with memory addresses; the second pass performs the final opcode generation.
* **Symbol Table Management:** Implemented an efficient mapping system to resolve symbolic labels and variables to specific physical memory addresses within the Hack RAM/ROM.
* **Lexical Analysis & Parsing:** Developed a modular parser to distinguish between A-instructions (addresses) and C-instructions (computations), handling mnemonics for the ALU, destination registers, and jump conditions.
* **Binary Encoding:** Performs direct bit-manipulation to encode symbolic mnemonics into the precise 16-bit binary format required by the Hack CPU.

### 🏗 Architecture
The project is built on a modular design, separating concerns between:
1. **Parser:** Handles string manipulation and instruction identification.
2. **Code Module:** Translates mnemonics into their binary equivalents.
3. **Symbol Table:** Manages label-to-address mappings.

### 🚀 Usage

**1. Clone and Prepare**
```
git clone [your-repo-link]
make build
```

**2. Assemble a File**
Ensure your `.asm` file is in the working directory:
```
./hack {filename}.asm
```

**3. Output**
The program generates an `out.hack` file containing the binary machine code and logs the translation process to the terminal.
