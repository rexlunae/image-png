//! Benchmarks for PNG encoding operations
//!
//! Usage example:
//! ```
//! $ cargo bench --bench encoder
//! $ cargo bench --bench encoder -- encode/rgb
//! ```

use std::io::Cursor;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, ColorType, Compression, Encoder};

fn bench_encoder_all(c: &mut Criterion) {
    // Benchmark different image sizes
    let sizes = [
        (64, 64, "64x64"),
        (256, 256, "256x256"),
        (512, 512, "512x512"),
        (1024, 1024, "1024x1024"),
    ];

    // Benchmark RGB encoding at different sizes
    let mut g = c.benchmark_group("encode/rgb");
    for &(width, height, label) in &sizes {
        bench_encode_rgb(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark RGBA encoding at different sizes
    let mut g = c.benchmark_group("encode/rgba");
    for &(width, height, label) in &sizes {
        bench_encode_rgba(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark grayscale encoding
    let mut g = c.benchmark_group("encode/grayscale");
    for &(width, height, label) in &sizes {
        bench_encode_grayscale(&mut g, width, height, label);
    }
    g.finish();

    // Benchmark different compression levels
    let mut g = c.benchmark_group("encode/compression");
    bench_encode_compression(&mut g, 512, 512);
    g.finish();

    // Benchmark different color types and bit depths
    let mut g = c.benchmark_group("encode/color-types");
    bench_encode_color_types(&mut g, 256, 256);
    g.finish();

    // Benchmark indexed/paletted images
    let mut g = c.benchmark_group("encode/indexed");
    bench_encode_indexed(&mut g, 256, 256);
    g.finish();
}

criterion_group!(benches, bench_encoder_all);
criterion_main!(benches);

fn bench_encode_rgb(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
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
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_encode_rgba(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32, label: &str) {
    let data = create_test_image_rgba(width, height);
    let bytes = (width * height * 4) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_encode_grayscale(
    g: &mut BenchmarkGroup<WallTime>,
    width: u32,
    height: u32,
    label: &str,
) {
    let data = create_test_image_grayscale(width, height);
    let bytes = (width * height) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Grayscale);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_encode_compression(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    g.throughput(Throughput::Bytes(bytes));

    // Fastest compression
    g.bench_function("fastest", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_compression(Compression::Fastest);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Fast compression
    g.bench_function("fast", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_compression(Compression::Fast);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Balanced compression
    g.bench_function("balanced", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_compression(Compression::Balanced);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_encode_color_types(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    // RGB 8-bit
    let data_rgb = create_test_image_rgb(width, height);
    g.throughput(Throughput::Bytes((width * height * 3) as u64));
    g.bench_function("rgb8", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data_rgb).unwrap();
        })
    });

    // RGBA 8-bit
    let data_rgba = create_test_image_rgba(width, height);
    g.throughput(Throughput::Bytes((width * height * 4) as u64));
    g.bench_function("rgba8", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data_rgba).unwrap();
        })
    });

    // Grayscale 8-bit
    let data_gray = create_test_image_grayscale(width, height);
    g.throughput(Throughput::Bytes((width * height) as u64));
    g.bench_function("gray8", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Grayscale);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data_gray).unwrap();
        })
    });

    // Grayscale with alpha 8-bit
    let data_gray_alpha = create_test_image_grayscale_alpha(width, height);
    g.throughput(Throughput::Bytes((width * height * 2) as u64));
    g.bench_function("gray_alpha8", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::GrayscaleAlpha);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data_gray_alpha).unwrap();
        })
    });

    // RGB 16-bit
    let data_rgb16 = create_test_image_rgb16(width, height);
    g.throughput(Throughput::Bytes((width * height * 6) as u64));
    g.bench_function("rgb16", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Sixteen);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data_rgb16).unwrap();
        })
    });
}

fn bench_encode_indexed(g: &mut BenchmarkGroup<WallTime>, width: u32, height: u32) {
    let (data, palette) = create_test_image_indexed(width, height);
    let bytes = (width * height) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("indexed8", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Indexed);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_palette(palette.clone());
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
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

fn create_test_image_grayscale_alpha(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 2) as usize);
    for y in 0..height {
        for x in 0..width {
            data.push(((x + y) % 256) as u8);
            data.push(255);
        }
    }
    data
}

fn create_test_image_rgb16(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::with_capacity((width * height * 6) as usize);
    for y in 0..height {
        for x in 0..width {
            let r = ((x * 256) % 65536) as u16;
            let g = ((y * 256) % 65536) as u16;
            let b = (((x + y) * 256) % 65536) as u16;
            data.extend_from_slice(&r.to_be_bytes());
            data.extend_from_slice(&g.to_be_bytes());
            data.extend_from_slice(&b.to_be_bytes());
        }
    }
    data
}

fn create_test_image_indexed(width: u32, height: u32) -> (Vec<u8>, Vec<u8>) {
    // Create a simple palette with 256 colors
    let mut palette = Vec::with_capacity(256 * 3);
    for i in 0..256 {
        palette.push(i as u8);
        palette.push(((255 - i) % 256) as u8);
        palette.push(((i * 2) % 256) as u8);
    }

    // Create indexed data
    let mut data = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            data.push(((x + y) % 256) as u8);
        }
    }

    (data, palette)
}
