//! Benchmarks for PNG streaming operations
//!
//! Usage example:
//! ```
//! $ cargo bench --bench streaming
//! ```

use std::io::{Cursor, Write};

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, ColorType, Encoder};

fn bench_streaming_all(c: &mut Criterion) {
    let sizes = [
        (256, 256, "256x256"),
        (512, 512, "512x512"),
        (1024, 1024, "1024x1024"),
    ];

    // Benchmark streaming write with different buffer sizes
    let mut g = c.benchmark_group("streaming/write");
    for &(width, height, label) in &sizes {
        bench_streaming_write(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark chunked writes (simulating progressive encoding)
    let mut g = c.benchmark_group("streaming/chunked");
    for &(width, height, label) in &sizes {
        bench_chunked_write(&mut g, width, height, label);
    }
    g.finish();
}

criterion_group!(benches, bench_streaming_all);
criterion_main!(benches);

fn bench_streaming_write(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            
            // Use stream_writer for more control
            let mut stream_writer = writer.stream_writer().unwrap();
            stream_writer.write_all(&data).unwrap();
            stream_writer.finish().unwrap();
        })
    });
}

fn bench_chunked_write(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;
    let row_size = (width * 3) as usize;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            
            // Write row by row to simulate progressive encoding
            let mut stream_writer = writer.stream_writer().unwrap();
            for row in data.chunks(row_size) {
                stream_writer.write_all(row).unwrap();
            }
            stream_writer.finish().unwrap();
        })
    });
}

// Helper functions to create test images

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
