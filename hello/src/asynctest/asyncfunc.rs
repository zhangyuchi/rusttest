use std::error::Error;
use std::thread::sleep;
use std::time;

struct Song {
    pub name: String,
    pub finish: bool,
}

impl Song{
    fn sing(&self) -> Result<(), Box<dyn Error>> {
        println!("sing: {} begin", self.name);
        sleep(time::Duration::from_secs(1));
        println!("sing: {} end", self.name);
        Ok(())
    }

    fn learn(&self) -> Result<(), Box<dyn Error>> {
        println!("learn: {} begin", self.name);
        sleep(time::Duration::from_secs(1));
        println!("learn: {} end", self.name);
        Ok(())
    }
}

async fn learn_song() -> Result<Song, Box<dyn Error>> {
    let song = Song{name:String::from("long river"), finish:true};
    song.learn()?;
    return Ok(song);
}

async fn sing_song(song: Song){
    song.sing().expect("fatal error: sing song");
    ()
}

async fn dance() {
    println!("dance begin");
    sleep(time::Duration::from_secs(1));
    println!("dance end");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    if let Ok(song) = learn_song().await{
        sing_song(song).await;
    }
    //println!("learn_and_sing done");
}

pub async fn async_main() {
    let f1 = learn_and_sing();
    println!("i am first");
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
    println!("async_main done");
}
