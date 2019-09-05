use futures::compat::Future01CompatExt;
use futures::future::{FutureExt, TryFutureExt};

pub fn async_main() {
    let f = async move {
        for i in 1..10000 {
            let _ = i * 2;
        }
        println!("asynctokio is over!");
    };

    tokio::run(f.boxed().unit_error().compat());
}