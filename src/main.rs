use rustdb::RustDB;
use std::time::Instant;

fn main() {
    let mut db = RustDB::open();

    db.put(b"person:1".to_vec(), b"Alice".to_vec());
    println!("get person:1 = {:?}", db.get(b"person:1"));

    db.put_with_secondary(
        b"person:2".to_vec(),
        b"Bob".to_vec(),
        b"Bob".to_vec(),
    );
    println!("get by secondary Bob = {:?}", db.get_by_secondary(b"Bob"));

    db.delete_with_secondary(b"person:2", b"Bob");
    println!("after delete Bob = {:?}", db.get_by_secondary(b"Bob"));

    let scan = db.scan(b"person:");
    println!("scan results = {:?}", scan);

    let duration = benchmark(&mut db, 100);
    println!("benchmark 100 ops = {:.2?}", duration);

    let mut tx = db.begin_transaction();
    tx.put(b"transaction:1".to_vec(), b"ValueTx".to_vec());
    assert_eq!(tx.get(b"transaction:1"), Some(b"ValueTx".to_vec()));
    tx.commit();
    println!("transaction committed = {:?}", db.get(b"transaction:1"));
}

fn benchmark(db: &mut RustDB, count: usize) -> std::time::Duration {
    let start = Instant::now();
    for i in 0..count {
        let key = format!("bench-{}", i).into_bytes();
        let value = format!("value-{}", i).into_bytes();
        db.put(key.clone(), value);
        let _ = db.get(&key);
    }
    start.elapsed()
}
