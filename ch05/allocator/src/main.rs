#![feature(allocator_api)]

use allocator::PassThruAllocator;

fn main() {
    let mut custom_alloc_vec = Vec::with_capacity_in(10, PassThruAllocator);

    (0..10).for_each(|i| custom_alloc_vec.push(i));

    println!("custom_alloc_vec={:?}", custom_alloc_vec);
}
