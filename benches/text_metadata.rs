//! Benchmarks for PNG text metadata operations
//!
//! Usage example:
//! ```
//! $ cargo bench --bench text_metadata
//! ```

use std::io::Cursor;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, ColorType, Encoder};

fn bench_text_metadata_all(c: &mut Criterion) {
    // Benchmark adding different types of text chunks
    let mut g = c.benchmark_group("text-metadata/add");
    bench_add_text_chunks(&mut g);
    g.finish();

    // Benchmark encoding with various amounts of text metadata
    let mut g = c.benchmark_group("text-metadata/encode");
    bench_encode_with_text(&mut g);
    g.finish();
}

criterion_group!(benches, bench_text_metadata_all);
criterion_main!(benches);

fn bench_add_text_chunks(g: &mut BenchmarkGroup<WallTime>) {
    let width = 256;
    let height = 256;
    let data = create_test_image_rgb(width, height);

    // Benchmark adding tEXt chunk (uncompressed Latin-1)
    g.bench_function("text-chunk", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder
                .add_text_chunk("Author".to_string(), "Test Author".to_string())
                .unwrap();
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Benchmark adding zTXt chunk (compressed Latin-1)
    g.bench_function("ztxt-chunk", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder
                .add_ztxt_chunk("Description".to_string(), "A longer description that will be compressed to save space in the PNG file.".to_string())
                .unwrap();
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Benchmark adding iTXt chunk (international text, UTF-8)
    g.bench_function("itxt-chunk", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder
                .add_itxt_chunk("Comment".to_string(), "Hello 世界! 🌍".to_string())
                .unwrap();
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });
}

fn bench_encode_with_text(g: &mut BenchmarkGroup<WallTime>) {
    let width = 256;
    let height = 256;
    let data = create_test_image_rgb(width, height);
    let bytes = (width * height * 3) as u64;

    // Benchmark encoding with no text metadata
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("no-text", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Benchmark encoding with multiple text chunks
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("multiple-text-chunks", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder
                .add_text_chunk("Title".to_string(), "Test Image".to_string())
                .unwrap();
            encoder
                .add_text_chunk("Author".to_string(), "Benchmark Suite".to_string())
                .unwrap();
            encoder
                .add_ztxt_chunk("Description".to_string(), "This is a comprehensive description of the test image used in benchmarking.".to_string())
                .unwrap();
            encoder
                .add_itxt_chunk("Copyright".to_string(), "© 2024 Test".to_string())
                .unwrap();
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&data).unwrap();
        })
    });

    // Benchmark with large text chunk
    let large_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(50);
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("large-text-chunk", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder
                .add_ztxt_chunk("LongDescription".to_string(), large_text.clone())
                .unwrap();
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
