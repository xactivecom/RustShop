# General Rust Language Advise

These are **Rust** language recommendations to follow during product development.

## Mutex
When using `tokio::sync::mutex::lock` know that it can become a contention point.
In the example where a temporary batch of events can be efficiently stored by sharding
by thread ID and use dashmap.

```
use hashmap::DashMap;
use std::thread;

lazy_static! {
	static ref SHARDED_BATCHES: DashMap<ThreadId, Vec<Event>> = DashMap::new();
}
fn handle_event(evt: Event) {
	let tid = thread::current().id();
	SHARDED_BATCHES.entry(tid).or_default().push(evt);
}
```


## Memory
Use **heaptrack** (https://github.com/KDE/heaptrack) to diagnose memory usage.

One possible culprit is allocations in `serde_json::Value` or too many `.clone()` methods.

Prefer use of `FxHashMap` instead of `BTreeMap`.


## Threading
The default thread allocation `num_cpus::get()` or even `crossbeam` can oversaturate the CPU
and result in unbalaned cores - idling and thrashing.

Don't freeze the async executor, so use `tokio::task::spawn_blocking` to offload CPU-bound work, such as:
```
tokio::task::spawn_blocking(move || {
	do_heavy_work(event);
});
```

## Main
An example architecture pseudo-code with no global locks, or clones is:
```
#[tokio::main]
async fn main() {
	let consumer = KafkaConsumer::new();

	while let Some(event) = consumer.next().await {
		tokio::spawn(process(event));
	}

	async fn process(event: Event) {
		let transformed = tokio::task::spawn_blocking(move || {
			do_heavy_work(event)
		}).await.unwrap();
		DB_POOL.execute(transformed).await;
	}
}
```
