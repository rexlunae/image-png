//! Benchmarks for PNG roundtrip operations (encode + decode)
//!
//! Usage example:
//! ```
//! $ cargo bench --bench roundtrip
//! ```

use std::io::Cursor;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

fn bench_roundtrip_all(c: &mut Criterion) {
    let sizes = [
        (128, 128, "128x128"),
        (512, 512, "512x512"),
        (1024, 1024, "1024x1024"),
    ];

    // Benchmark RGB roundtrip
    let mut g = c.benchmark_group("roundtrip/rgb");
    for &(width, height, label) in &sizes {
        bench_roundtrip_rgb(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark RGBA roundtrip
    let mut g = c.benchmark_group("roundtrip/rgba");
    for &(width, height, label) in &sizes {
        bench_roundtrip_rgba(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark grayscale roundtrip
    let mut g = c.benchmark_group("roundtrip/grayscale");
    for &(width, height, label) in &sizes {
        bench_roundtrip_grayscale(&mut g, width, height, label);
    }
    g.finish();
}

criterion_group!(benches, bench_roundtrip_all);
criterion_main!(benches);

fn bench_roundtrip_rgb(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
    let original_data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            // Encode
            let mut encoded = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut encoded, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&original_data).unwrap();
            drop(writer);

            // Decode
            let encoded_data = encoded.into_inner();
            let decoder = Decoder::new(Cursor::new(encoded_data));
            let mut reader = decoder.read_info().unwrap();
            let mut decoded_data = vec![0; reader.output_buffer_size().unwrap()];
            reader.next_frame(&mut decoded_data).unwrap();
        })
    });
}

fn bench_roundtrip_rgba(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
    let original_data = create_test_image_rgba(width, height);
    let bytes = (width * height * 4) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            // Encode
            let mut encoded = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut encoded, width, height);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&original_data).unwrap();
            drop(writer);

            // Decode
            let encoded_data = encoded.into_inner();
            let decoder = Decoder::new(Cursor::new(encoded_data));
            let mut reader = decoder.read_info().unwrap();
            let mut decoded_data = vec![0; reader.output_buffer_size().unwrap()];
            reader.next_frame(&mut decoded_data).unwrap();
        })
    });
}

fn bench_roundtrip_grayscale(
    g: &mut BenchmarkGroup<WallTime>,
    width: u32,
    height: u32,
    label: &str,
) {
    let original_data = create_test_image_grayscale(width, height);
    let bytes = (width * height) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            // Encode
            let mut encoded = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut encoded, width, height);
            encoder.set_color(ColorType::Grayscale);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&original_data).unwrap();
            drop(writer);

            // Decode
            let encoded_data = encoded.into_inner();
            let mut decoder = Decoder::new(Cursor::new(encoded_data));
            decoder.set_transformations(Transformations::IDENTITY);
            let mut reader = decoder.read_info().unwrap();
            let mut decoded_data = vec![0; reader.output_buffer_size().unwrap()];
            reader.next_frame(&mut decoded_data).unwrap();
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

fn create_test_image_rgba(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        for x in 0..width {
            data.push((x % 256) as u8);
            data.push((y % 256) as u8);
            data.push(((x + y) % 256) as u8);
            data.push(255);
        }
    }
    data
}

fn create_test_image_grayscale(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            data.push(((x + y) % 256) as u8);
        }
    }
    data
}
