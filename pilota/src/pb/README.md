The files in this directory are copied from the [Prost](https://github.com/tokio-rs/prost) project and modified for performance.
For zero copy, we use `Bytes` for decode and `LinkedBytes` for encode.