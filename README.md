# Rust-Tutorial
### Rust Hands On
#### Following the O'Reilly Programing Rust (2nd Edition) Book

#### Current Setup Version:
```markdown
cargo 1.72.1
rustc 1.72.1 (d5c2e9c34 2023-09-13) (Homebrew)
rustdoc 1.72.1 (d5c2e9c34 2023-09-13) (Homebrew)
```

### New Project Genaration CMD for Greatest Common Divisor(GCD)
```console
cargo new gcd
```


**Tutorials**
1. Greatest Common Divisor(GCD) using Euclid's algorithm (Method, Unit test) [**30/Aug/2024**]
    input -> 
    ```console
    cargo test
    ```

    output -> 
    ```console
    test test_gcd ... ok
    ```
2. Greatest Common Divisor(GCD) using Command line argument [**06/Sep/2024**]

   input ->
   
   ```console
    cargo run 42 56
    ```
    output -> The greatest common divisor of [42, 56] is 14

    input ->
    
    ```console
    cargo run
    ```
    output -> 
    
    ```console
    Usage: gcd number ...
    ```
3. Greatest Common Divisor(GCD) using Web Pages [**07/Sep/2024**]
    ```console
    [dependencies]
    actix-web= "4"
    serde = { version = "1.0", features = ["derive"] }
    ```


