use futures::executor::block_on;

async fn learn_song() -> Song {}
async fn sing_song(song: Song) {}
async fn dance() {}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
/*async fn hello_world() {
    println!("Hello world");
}

fn main() {
    let future = hello_world();
    block_on(future);
}*/
