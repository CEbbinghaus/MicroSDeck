use std::{fs, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glob::glob;

pub fn bench_read_dir(folder: &str) -> usize {
    std::fs::read_dir(folder)
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or_default().eq("acf"))
        .count()
}

pub fn bench_glob(folder: &str) -> usize {
    glob(&format!("{}/*.acf", folder)).unwrap().count()
}


pub fn bench_hashing(folder: &str) -> u64 {
    let file_metadata: Vec<_> = fs::read_dir(folder)
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().unwrap_or_default().eq("a"))
        .filter_map(|f| fs::metadata(f.path()).ok())
        .collect();
    
    let mut s = DefaultHasher::new();
    
    for metadata in file_metadata {
        metadata.len().hash(&mut s);
        metadata.modified().expect("Last Modified time to exist").hash(&mut s); 
    }

    s.finish()
}



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench read dir", |b| b.iter(|| bench_read_dir(black_box("/run/media/mmcblk0p1/steamapps/"))));
    c.bench_function("bench glob", |b| b.iter(|| bench_glob(black_box("/run/media/mmcblk0p1/steamapps/"))));
    c.bench_function("bench hashing", |b| b.iter(|| bench_hashing(black_box("/run/media/mmcblk0p1/steamapps/"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);