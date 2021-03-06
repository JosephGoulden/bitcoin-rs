extern crate chain;
extern crate criterion;
extern crate miner;
extern crate primitives;
extern crate serialization as ser;

use crate::miner::NonZeroFeeCalculator;
use chain::{OutPoint, Transaction, TransactionInput};
use criterion::{criterion_group, criterion_main, Criterion};
use miner::{MemoryPool, MemoryPoolOrderingStrategy};
use primitives::bytes::Bytes;
use std::collections::VecDeque;

fn prepare_independent_transactions(n: usize) -> VecDeque<Transaction> {
	(0..n)
		.map(|nonce| Transaction {
			version: nonce as i32,
			inputs: vec![],
			outputs: vec![],
			lock_time: 0,
		})
		.collect()
}

fn prepare_dependent_transactions(n: usize) -> VecDeque<Transaction> {
	let previous_transaction = Transaction {
		version: 0 as i32,
		inputs: vec![],
		outputs: vec![],
		lock_time: 0,
	};

	let mut previous_transaction_hash = previous_transaction.hash();
	let mut result = VecDeque::new();
	result.push_back(previous_transaction);
	result.extend((0..n).map(|_nonce| {
		let transaction = Transaction {
			version: 0,
			inputs: vec![TransactionInput {
				previous_output: OutPoint {
					hash: previous_transaction_hash.clone(),
					index: 0,
				},
				script_sig: Bytes::new_with_len(0),
				sequence: 0,
				script_witness: vec![],
			}],
			outputs: vec![],
			lock_time: 0,
		};
		previous_transaction_hash = transaction.hash();
		transaction
	}));
	result
}

// test benchmarks::memory_pool_insert_independent_transactions ... bench:       117,148 ns/iter (+/- 12)
fn memory_pool_insert_independent_transactions(c: &mut Criterion) {
	let iterations = 100;
	let mut pool = MemoryPool::new();
	let transactions = prepare_independent_transactions(iterations);
	c.bench_function("memory_pool_insert_independent_transactions", |b| {
		b.iter(|| {
			let mut transactions = transactions.clone();
			(0..iterations).for_each(|_| pool.insert_verified(transactions.pop_front().unwrap().into(), &NonZeroFeeCalculator {}))
		})
	});
}

// test benchmarks::memory_pool_insert_descendant_transaction   ... bench:       675,116 ns/iter (+/- 288)
fn memory_pool_insert_descendant_transaction(c: &mut Criterion) {
	let iterations = 100;
	let transactions = prepare_dependent_transactions(iterations);

	c.bench_function("memory_pool_insert_descendant_transaction", |b| {
		b.iter(|| {
			let mut transactions = transactions.clone();
			let mut pool = MemoryPool::new();
			pool.insert_verified(transactions.pop_front().unwrap().into(), &NonZeroFeeCalculator {});
			(0..iterations).for_each(|_| pool.insert_verified(transactions.pop_front().unwrap().into(), &NonZeroFeeCalculator {}))
		})
	});
}

// test benchmarks::memory_pool_insert_ancestor_transaction     ... bench:     27,232,305 ns/iter (+/- 11,249)
// very slow due to weird usage scenario:
// (1) transactions inserted to memory pool are verified
// (2) verified => ancestors already verified
// (3) ancestors verified => they are already in memory pool (not this case) or in database (why inserting to memorypool then)
fn memory_pool_insert_ancestor_transaction(c: &mut Criterion) {
	let iterations = 100;
	let transactions = prepare_dependent_transactions(iterations);
	c.bench_function("memory_pool_insert_ancestor_transaction", |b| {
		b.iter(|| {
			let mut transactions = transactions.clone();
			let mut pool = MemoryPool::new();
			pool.insert_verified(transactions.pop_front().unwrap().into(), &NonZeroFeeCalculator {});
			(0..iterations).for_each(|_| pool.insert_verified(transactions.pop_back().unwrap().into(), &NonZeroFeeCalculator {}))
		})
	});
}

// test benchmarks::memory_pool_remove_independent_in_order     ... bench:         872 ns/iter (+/- 47)
fn memory_pool_remove_independent_in_order(c: &mut Criterion) {
	let iterations = 100;
	let mut pool = MemoryPool::new();
	for transaction in prepare_independent_transactions(iterations) {
		pool.insert_verified(transaction.into(), &NonZeroFeeCalculator {})
	}
	c.bench_function("memory_pool_remove_independent_in_order", |b| {
		b.iter(|| {
			(0..iterations).for_each(|_| {
				pool.remove_with_strategy(MemoryPoolOrderingStrategy::ByTimestamp);
			})
		})
	});
}

// test benchmarks::memory_pool_remove_dependent_in_order       ... bench:         880 ns/iter (+/- 111)
fn memory_pool_remove_dependent_in_order(c: &mut Criterion) {
	let iterations = 100;
	let mut pool = MemoryPool::new();
	for transaction in prepare_dependent_transactions(iterations) {
		pool.insert_verified(transaction.into(), &NonZeroFeeCalculator {})
	}
	c.bench_function("memory_pool_remove_dependent_in_order", |b| {
		b.iter(|| {
			(0..iterations).for_each(|_| {
				pool.remove_with_strategy(MemoryPoolOrderingStrategy::ByTimestamp);
			})
		})
	});
}

criterion_group!(
	benches,
	memory_pool_insert_ancestor_transaction,
	memory_pool_insert_descendant_transaction,
	memory_pool_insert_independent_transactions,
	memory_pool_remove_dependent_in_order,
	memory_pool_remove_independent_in_order
);
criterion_main!(benches);
