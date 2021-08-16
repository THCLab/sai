# sai

sai provides a compact text representation of digests of data. 
It supports multiple hash algorithms.

Self Adressing Identifeir is base64 encoded hash of data, prepended with
one or two letter prefix, called derivation code. Derivation code determines
which hash algorithm was used to derive the digest.

| deivation code| digest type 		| code lenngth 	| identifier length	|
|---------------|-------------------|---------------|-------------------|
| E				| Blake3-256 Digest | 1				| 44 				|
| F 			| Blake2b-256 Digest| 1				| 44				|
| G				| Blake2s-256 Digest| 1				| 44				|
| H				| SHA3-256 Digest 	| 1				| 44				|
| I				| SHA2-256 Digest	| 1				| 44				|
| 0D			| Blake3-512 Digest | 2				| 88				|
| 0E			| Blake2b-512 Digest| 2				| 88				|
| 0F			| SHA3-512 Digest 	| 2				| 88				|
| 0G			| SHA2-512 Digest	| 2				| 88				|

## Usage

To derive Self Addressing Identifier, `SelfAddressing` enum can be used. It
determines hashing algorithm used for Identifier derivation.  Here are its
possible values: 

```rust
pub enum SelfAddressing {
    Blake3_256,
    Blake2B256(Vec<u8>),
    Blake2S256(Vec<u8>),
    SHA3_256,
    SHA2_256,
    Blake3_512,
    SHA3_512,
    Blake2B512,
    SHA2_512,
}
```

`verify_bindings` function checks if provided data matches digest encoded in Self Adressing Identifier.

### Example
```rust
let data = "hello there";
let sai = SelfAddressing::Blake3_256.derive(data.as_bytes());

assert_eq!(format!("{}", sai), "E2bCqepXGid_9s1nSEyKk4kljbl3GULx5JjkFvIwJ8sk");
assert!(sai.verify_binding(data.as_bytes()));
assert!(!sai.verify_binding("wrong data".as_bytes()));
```

## wasm
`bindings/sai_wasm` directory contains a project which allows to compile sai library to wasm.