use loom::{
    sync::{
        atomic::{
            AtomicUsize,
            Ordering::{Acquire, Relaxed, Release},
        },
        Arc,
    },
    thread,
};

#[test]
#[should_panic]
fn buggy_concurrent_inc() {
    loom::model(|| {
        let num = Arc::new(AtomicUsize::new(0));

        let threads: Vec<_> = (0..2)
            .map(|_| {
                let num = num.clone();
                thread::spawn(move || {
                    let curr = num.load(Acquire);
                    // this is a bug! this is not atomic!
                    num.store(curr + 1, Release);

                    // fix
                    // num.fetch_add(1, Relaxed);
                })
            })
            .collect();

        for th in threads {
            th.join().unwrap();
        }

        assert_eq!(2, num.load(Relaxed));
    });
}
