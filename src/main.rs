use futures::executor::block_on;
use futures::future::FutureExt;
use std::io::{stdin, stdout, Write};
use std::time::Duration;
async fn wait_for_input(){
    use async_std::io::stdin;
    println!("Press enter to end round early");
    let mut input = String::new();
    stdin().read_line(&mut input).await.expect("Failed to read line");
}

async fn countdown_30(){
    for i in 0..30001 {
        if i % 1000 == 0 {
            print!("\r{} ", 30-i/1000);
            stdout().flush().unwrap();
        }
        async_std::task::sleep(Duration::from_millis(1)).await;
    }
}

async fn async_main(){
    let t1 = wait_for_input().fuse();
    let t2 = countdown_30().fuse();

    futures::pin_mut!(t1, t2);

    futures::select! {
        () = t1 => println!("task one completed first"),
        () = t2 => println!("task two completed first"),
    }
}
fn main() {
    println!("How many larges would you like? (0-4)");

    let mut larges = String::new();
    stdin().read_line(&mut larges).expect("Failed to read line");

    let larges: u32 = larges.trim().parse().expect("Please input a number next time");

    let round = countdown::Round::new(larges);
    println!("Your round is: {}", round);

    
    block_on(async_main());

    let solution = round.solve();
    println!("\nSolution is:\n{}", solution);
}