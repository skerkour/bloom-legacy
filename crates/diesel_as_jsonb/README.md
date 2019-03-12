# Diesel As JSONB (for PG)

From https://github.com/terry90/diesel_as_jsonb

## Usage

```rust
#[derive(AsJsonb)]]
struct Something {
    thing: String,
}

struct Wrapper {
    things: Vec<Something> // For JSONB[]
    thing: Something // For JSONB
}
```
