# Getting started with benchmarking

To run the benchmarks you need a nightly rust toolchain.
Then you launch it with

    rustup run nightly cargo bench --features=benchmarks

## Available Benchmarks

### Decoder Benchmarks
- **decoder**: Benchmarks PNG decoding with various image sizes and formats
  ```
  cargo bench --bench decoder
  ```

### Encoder Benchmarks
- **encoder**: Comprehensive encoding benchmarks covering different image sizes, color types, bit depths, and compression levels
  ```
  cargo bench --bench encoder
  cargo bench --bench encoder -- encode/rgb      # RGB encoding only
  cargo bench --bench encoder -- encode/rgba     # RGBA encoding only
  cargo bench --bench encoder -- compression     # Compression levels
  cargo bench --bench encoder -- color-types     # Different color types
  ```

### Roundtrip Benchmarks
- **roundtrip**: Benchmarks encode+decode roundtrip operations
  ```
  cargo bench --bench roundtrip
  ```

### Streaming Benchmarks
- **streaming**: Benchmarks streaming write operations
  ```
  cargo bench --bench streaming
  ```

### Filter Benchmarks
- **filter_encode**: Benchmarks different filter types during encoding
  ```
  cargo bench --bench filter_encode
  ```
- **unfilter**: Benchmarks filter operations during decoding (requires `--features=benchmarks`)
  ```
  cargo bench --bench unfilter --features=benchmarks
  ```

### Text Metadata Benchmarks
- **text_metadata**: Benchmarks PNG text chunk operations (tEXt, zTXt, iTXt)
  ```
  cargo bench --bench text_metadata
  ```

### APNG (Animated PNG) Benchmarks
- **apng**: Benchmarks animated PNG encoding with different frame operations
  ```
  cargo bench --bench apng
  cargo bench --bench apng -- simple        # Simple animations
  cargo bench --bench apng -- operations    # Frame operations (blend, dispose)
  ```

### Internal Optimization Benchmarks
These require the `benchmarks` feature:
- **adam7**: Benchmarks Adam7 interlaced image expansion
  ```
  cargo bench --bench adam7 --features=benchmarks
  ```
- **expand_paletted**: Benchmarks palette expansion operations
  ```
  cargo bench --bench expand_paletted --features=benchmarks
  ```

## Running All Benchmarks

To run all benchmarks:
```
cargo bench --features=benchmarks
```

## Saving and Comparing Baselines

You can save benchmark results as a baseline and compare against it later:

```bash
# Save a baseline
cargo bench --bench encoder -- --save-baseline my-baseline

# Make some changes to the code...

# Compare against the baseline
cargo bench --bench encoder -- --baseline my-baseline
```

## Tips

- Use `--sample-size N` to control the number of samples (default is 100):
  ```
  cargo bench --bench encoder -- --sample-size 10
  ```

- Filter benchmarks by name:
  ```
  cargo bench --bench encoder -- rgb
  cargo bench -- roundtrip/512x512
  ```

- Generate plots (requires gnuplot):
  ```
  cargo bench --features=benchmarks
  # Results will be in target/criterion/
  ```

