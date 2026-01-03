//! Benchmarks for APNG (Animated PNG) encoding
//!
//! Usage example:
//! ```
//! $ cargo bench --bench apng
//! ```

use std::io::Cursor;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, Throughput,
};
use png::{BitDepth, BlendOp, ColorType, DisposeOp, Encoder};

fn bench_apng_all(c: &mut Criterion) {
    // Benchmark encoding simple animations
    let mut g = c.benchmark_group("apng/simple");
    bench_simple_animation(&mut g, 64, 64, 4, "64x64-4frames");
    bench_simple_animation(&mut g, 128, 128, 8, "128x128-8frames");
    bench_simple_animation(&mut g, 256, 256, 4, "256x256-4frames");
    g.finish();

    // Benchmark different frame operations
    let mut g = c.benchmark_group("apng/operations");
    bench_frame_operations(&mut g);
    g.finish();
}

criterion_group!(benches, bench_apng_all);
criterion_main!(benches);

fn bench_simple_animation(
    g: &mut BenchmarkGroup<WallTime>,
    width: u32,
    height: u32,
    num_frames: u32,
    label: &str,
) {
    let frames = create_animation_frames(width, height, num_frames);
    let bytes = (width * height * 3 * num_frames) as u64;

    g.throughput(Throughput::Bytes(bytes));
    g.bench_function(label, |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap(); // 0 = loop forever
            encoder.set_frame_delay(10, 100).unwrap(); // 10/100 = 0.1 second per frame

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });
}

fn bench_frame_operations(g: &mut BenchmarkGroup<WallTime>) {
    let width = 128;
    let height = 128;
    let num_frames = 4;
    let frames = create_animation_frames(width, height, num_frames);
    let bytes = (width * height * 3 * num_frames) as u64;

    // Benchmark with BlendOp::Source (replace frame)
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("blend-source", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap();
            encoder.set_frame_delay(10, 100).unwrap();
            encoder.set_blend_op(BlendOp::Source).unwrap();

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });

    // Benchmark with BlendOp::Over (alpha blend)
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("blend-over", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap();
            encoder.set_frame_delay(10, 100).unwrap();
            encoder.set_blend_op(BlendOp::Over).unwrap();

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });

    // Benchmark with DisposeOp::None (leave frame)
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("dispose-none", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap();
            encoder.set_frame_delay(10, 100).unwrap();
            encoder.set_dispose_op(DisposeOp::None).unwrap();

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });

    // Benchmark with DisposeOp::Background (clear to background)
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("dispose-background", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap();
            encoder.set_frame_delay(10, 100).unwrap();
            encoder.set_dispose_op(DisposeOp::Background).unwrap();

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });

    // Benchmark with DisposeOp::Previous (restore previous)
    g.throughput(Throughput::Bytes(bytes));
    g.bench_function("dispose-previous", |b| {
        b.iter(|| {
            let mut output = Cursor::new(Vec::new());
            let mut encoder = Encoder::new(&mut output, width, height);
            encoder.set_color(ColorType::Rgb);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_animated(num_frames, 0).unwrap();
            encoder.set_frame_delay(10, 100).unwrap();
            encoder.set_dispose_op(DisposeOp::Previous).unwrap();

            let mut writer = encoder.write_header().unwrap();

            for frame_data in &frames {
                writer.write_image_data(frame_data).unwrap();
            }
        })
    });
}

// Helper function to create animation frames

fn create_animation_frames(width: u32, height: u32, num_frames: u32) -> Vec<Vec<u8>> {
    let mut frames = Vec::with_capacity(num_frames as usize);

    for frame_idx in 0..num_frames {
        let mut frame_data = Vec::with_capacity((width * height * 3) as usize);
        let offset = (frame_idx * 32) as u8;

        for y in 0..height {
            for x in 0..width {
                frame_data.push((x as u8).wrapping_add(offset));
                frame_data.push((y as u8).wrapping_add(offset));
                frame_data.push(((x + y) as u8).wrapping_add(offset));
            }
        }

        frames.push(frame_data);
    }

    frames
}
