#[macro_use]

extern crate bencher;
extern crate cryptonight;
use bencher::Bencher;
use cryptonight::hash;

fn benchmark_hash_43_1(bench: &mut Bencher){
    let bytes = [1u8; 43];
    bench.iter(|| hash(&bytes, bytes.len(), 1));
}

fn benchmark_hash_1k_1(bench: &mut Bencher){
    let bytes = [1u8; 1024];
    bench.iter(|| hash(&bytes, bytes.len(), 1));
}

fn benchmark_hash_64k_1(bench: &mut Bencher){
    let bytes = [1u8; 65536];
    bench.iter(|| hash(&bytes, bytes.len(), 1));
}

fn benchmark_hash_43_0(bench: &mut Bencher){
    let bytes = [1u8; 43];
    bench.iter(|| hash(&bytes, bytes.len(), 0));
}

fn benchmark_hash_1k_0(bench: &mut Bencher){
    let bytes = [1u8; 1024];
    bench.iter(|| hash(&bytes, bytes.len(), 0));
}

fn benchmark_hash_64k_0(bench: &mut Bencher){
    let bytes = [1u8; 65536];
    bench.iter(|| hash(&bytes, bytes.len(), 0));
}

benchmark_group!(benches, 
benchmark_hash_43_1, 
benchmark_hash_1k_1, 
benchmark_hash_64k_1,
benchmark_hash_43_0,
benchmark_hash_1k_0,
benchmark_hash_64k_0);
benchmark_main!(benches);
