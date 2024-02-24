# Serialize and deserialize structs to and from byte arrays.

Uses a fixed-index format with little-endian bytes.

There is lots of missing functionality: Please submit an issue or PR if you find something missing.

Example struct:
```rust
#[derive(Clone, Copy)]
#[repr(u8)]
enum Test {
    A = 12,
    B = 14,
}

#[derive(SerBytes)]
struct SerTest {
    pub a: u32,
    pub b: f32,
    pub c: Test,
    pub d: u16,
}

// ...

let a = SerTest {
    a: 69_123,
    b: 1.232,
    c: Test::A,
    d: 1_000,
};

println!("Serialized: {:?}", a.serialize());
// Results: [3, 14, 1, 0, 45, 178, 157, 63, 12, 232]
```

Example serialization:
