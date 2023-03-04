Example of deserializable trait objects. It is using [`typetag`](https://github.com/dtolnay/typetag)
for painless serialization of ``&dyn Trait`` trait objects and serialization + deserialization of `Box<dyn Trait>` trait objects.

```rust
...

fn main() {
    let yaml = "
        - name: uppercase
        - name: my_custom
          x: 1
          y: 2
    ";

    let values: Vec<Box<dyn Transformer>> = serde_yaml::from_str(yaml).unwrap();

    for value in values {
        value.transform();
    }
}
```

```bash
$ cargo run
uppercase
my_custom: x=1 y=2
```
