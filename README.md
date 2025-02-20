# CKB Easy

CKB Easy is an experimental project that aims to provide human-friendly interfaces for common CKB operations. Note that CKB Easy is not a complete SDK, but only implements the CKB functions that I am interested in.

```toml
[dependencies]
ckbez = "0.120"
```

Features:

- Made the damn molecule, in a human way
- Implemented a unit testing framework

## Usage

**examples/molecule.rs**

Deserialize a script, modify its args, and reserialize it. If you use the native molecule library from ckb, this will be the worst punishment in hell.

```sh
$ cargo run --example molecule

# Raw script in hex: 3900000010000000300000003100000082d76d1b75fe2fd9a27dfbaa65a039221a380d76c926f378d3f81cf3e7e13f2e010400000000010203
# Raw script in obj: Script { code_hash: [130, 215, 109, 27, 117, 254, 47, 217, 162, 125, 251, 170, 101, 160, 57, 34, 26, 56, 13, 118, 201, 38, 243, 120, 211, 248, 28, 243, 231, 225, 63, 46], hash_type: 1, args: [0, 1, 2, 3] }
# New script in obj: Script { code_hash: [130, 215, 109, 27, 117, 254, 47, 217, 162, 125, 251, 170, 101, 160, 57, 34, 26, 56, 13, 118, 201, 38, 243, 120, 211, 248, 28, 243, 231, 225, 63, 46], hash_type: 1, args: [3, 2, 1, 0] }
# New script in hex: 3900000010000000300000003100000082d76d1b75fe2fd9a27dfbaa65a039221a380d76c926f378d3f81cf3e7e13f2e010400000003020100
```

**examples/unittest.rs**

Create a transaction whose input is locked by the exit_0 contract, and execute it.

```sh
$ cargo run --example unittest

# All cycles: 0.0 M
```

## Licences

MIT
