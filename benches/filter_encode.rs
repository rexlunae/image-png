//! Benchmarks for PNG filter operations during encoding
//!
//! Usage example:
//! ```
//! $ cargo bench --bench filter_encode
//! ```

use std::io::Cursor;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, ColorType, Encoder, Filter};

fn bench_filter_encode_all(c: &mut Criterion) {
    let width = 512;
    let height = 512;

    // Benchmark different filter types
    let mut g = c.benchmark_group("filter-encode");
    bench_filter_type(&mut g, width, height, Filter::NoFilter, "none");
    bench_filter_type(&mut g, width, height, Filter::Sub, "sub");
    bench_filter_type(&mut g, width, height, Filter::Up, "up");
    bench_filter_type(&mut g, width, height, Filter::Avg, "avg");
    bench_filter_type(&mut g, width, height, Filter::Paeth, "paeth");
    g.finish();

    // Benchmark adaptive filtering (default)
    let mut g = c.benchmark_group("filter-encode/adaptive");
    bench_adaptive_filter(&mut g, width, height);
    g.finish();
}

criterion_group!(benches, bench_filter_encode_all);
criterion_main!(benches);

fn bench_filter_type(
    g: &mut BenchmarkGroup<WallTime>,
    width: u32,
    height: u32,
    filter: Filter,
    label: &str,
) {
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_filter(filter);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_adaptive_filter(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("adaptive", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            // Adaptive filter is the default - encoder automatically chooses best filter per row
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

// Helper function to create test image

fn create_test_image_rgb(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            data.push((x % 256) as u8);
            data.push((y % 256) as u8);
            data.push(((x + y) % 256) as u8);
        }
    }
    data
}
