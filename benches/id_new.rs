use criterion::{Criterion, criterion_group, criterion_main};
use logic::id::{DimensionRange, SpaceTimeId};
use std::hint::black_box;

fn bench_spacetimeid_valid_full_range(c: &mut Criterion) {
    c.bench_function("valid: full range (x/y/f = LimitRange)", |b| {
        b.iter(|| {
            let z = black_box(5);
            let max_xy = (1u64 << z) - 1;
            let x = DimensionRange::LimitRange(0, max_xy);
            let y = DimensionRange::LimitRange(0, max_xy);
            let f = DimensionRange::LimitRange(-16, 15);
            let i = 60;
            let t = DimensionRange::LimitRange(0, 10);

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_ok());
        });
    });
}

fn bench_spacetimeid_valid_any(c: &mut Criterion) {
    c.bench_function("valid: all Any", |b| {
        b.iter(|| {
            let z = black_box(5);
            let x = DimensionRange::Any;
            let y = DimensionRange::Any;
            let f = DimensionRange::Any;
            let i = 60;
            let t = DimensionRange::Any;

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_ok());
        });
    });
}

fn bench_spacetimeid_valid_single(c: &mut Criterion) {
    c.bench_function("valid: all Single", |b| {
        b.iter(|| {
            let z = black_box(5);
            let x = DimensionRange::Single(10);
            let y = DimensionRange::Single(20);
            let f = DimensionRange::Single(0);
            let i = 60;
            let t = DimensionRange::Single(100);

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_ok());
        });
    });
}

fn bench_spacetimeid_invalid_z_overflow(c: &mut Criterion) {
    c.bench_function("invalid: z overflow (z >= 32)", |b| {
        b.iter(|| {
            let z = black_box(32);
            let x = DimensionRange::Single(0);
            let y = DimensionRange::Single(0);
            let f = DimensionRange::Single(0);
            let i = 60;
            let t = DimensionRange::Single(0);

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_err());
        });
    });
}

fn bench_spacetimeid_invalid_xy_range(c: &mut Criterion) {
    c.bench_function("invalid: x out of range", |b| {
        b.iter(|| {
            let z = black_box(5);
            let x = DimensionRange::Single(1000); // out of 2^5 = 32 range
            let y = DimensionRange::Single(0);
            let f = DimensionRange::Single(0);
            let i = 60;
            let t = DimensionRange::Single(0);

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_err());
        });
    });
}

fn bench_spacetimeid_invalid_f_range(c: &mut Criterion) {
    c.bench_function("invalid: f out of range", |b| {
        b.iter(|| {
            let z = black_box(5);
            let x = DimensionRange::Single(0);
            let y = DimensionRange::Single(0);
            let f = DimensionRange::Single(1000); // out of -32 to 31
            let i = 60;
            let t = DimensionRange::Single(0);

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_err());
        });
    });
}

fn bench_spacetimeid_invalid_t_when_i_zero(c: &mut Criterion) {
    c.bench_function("invalid: i == 0 and t != Any", |b| {
        b.iter(|| {
            let z = black_box(5);
            let x = DimensionRange::Single(0);
            let y = DimensionRange::Single(0);
            let f = DimensionRange::Single(0);
            let i = 0;
            let t = DimensionRange::Single(10); // must be Any when i == 0

            let result = SpaceTimeId::new(z, f, x, y, i, t);
            assert!(result.is_err());
        });
    });
}

criterion_group!(
    benches,
    bench_spacetimeid_valid_full_range,
    bench_spacetimeid_valid_any,
    bench_spacetimeid_valid_single,
    bench_spacetimeid_invalid_z_overflow,
    bench_spacetimeid_invalid_xy_range,
    bench_spacetimeid_invalid_f_range,
    bench_spacetimeid_invalid_t_when_i_zero
);
criterion_main!(benches);
