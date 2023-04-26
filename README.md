
liquid-php
=========

An experiment to run [liquid-rust](https://crates.io/crates/liquid) as a PHP native extension.

# Usage

```
cargo build
php -d extension=./target/debug/libliquid_php.so examples/basic.php
```
