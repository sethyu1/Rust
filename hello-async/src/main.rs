// use trpl::{Either, Html};


// async fn page_title(url: &str) -> Option<String> {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;
//     Html::parse(&response_text)
//         .select_first("title")
//         .map(|title_element| title_element.inner_html())
// }


// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

// fn main() {
//     let args: Vec<String> = std::env::args().collect();
    
//     trpl::run(async {
//     let title_fut_1 = page_title(&args[1]);
//     let titlte_fut_2 = page_title(&args[2]);

//     let (url, maybe_title) =
//         match trpl::race(title_fut_1, titlte_fut_2).await {
//             Either::Left(left) => left,
//             Either::Right(right) => right,
//         };
    
//     println!("{url} returned first");
//     match maybe_title {
//         Some(title) => println!("Its page title is '{title}'"),
//         None => println!("Its page title could not be parsed."),
//     }
// })
// }

// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let text = trpl::get(url).await.text().await;
//     let title = Html::parse(&text)
//         .select_first("title")
//         .map(|title|title.inner_html());
//     (url, title)
// }



// fn main() {
//     trpl::run(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         handle.await.unwrap();

//         println!("All tasks completed!");
//     });
// }

// fn main() {
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         trpl::join(fut1, fut2).await;
//     });

// }

// use std::time::Duration;
// use std::pin::Pin;

// fn main() {
//     trpl::run(async {
//                 let (tx, mut rx) = trpl::channel();

//         let tx1 = tx.clone();
//         let tx1_fut = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx1.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         let tx_fut = async move {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(1500)).await;
//             }
//         };

//         let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
//             vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];


//         trpl::join_all(futures).await;


//     })
// }
// use std::time::{Duration, Instant};
// // use std::thread;


// // fn slow(name: &str, ms: u64) {
// //     thread::sleep(Duration::from_millis(ms));
// //     println!("'{name}' ran for {ms}ms");
// // }
// fn main() {
//     trpl::run(async {

//         let one_ns = Duration::from_nanos(1);
//         let start = Instant::now();
//         async {
//             for _ in 1..1000 {
//                 trpl::sleep(one_ns).await;
//             }
//         }
//         .await;
//         let time = Instant::now() - start;
//         println!(
//             "'sleep' version finished after {} seconds.",
//             time.as_secs_f32()
//         );

//         let start = Instant::now();
//         async {
//             for _ in 1..1000 {
//                 trpl::yield_now().await;
//             }
//         }
//         .await;
//         let time = Instant::now() - start;
//         println!(
//             "'yield' version finished after {} seconds.",
//             time.as_secs_f32()
//         );

//     })
//     }

// use trpl::Either;

// fn main() {
//     trpl::run(async {
//         let slow = async {
//             trpl::sleep(Duration::from_secs(5)).await;
//             "Finally finished"
//         };

//         match timeout(slow, Duration::from_secs(2)).await {
//             Ok(message) => println!("Succeeded with '{message}'"),
//             Err(duration) => {
//                 println!("Failed after {} seconds", duration.as_secs())
//             }
//         }
//     });
// }

// async fn timeout<F: Future>(
//     future_to_try: F,
//     max_time: Duration,
// ) -> Result<F::Output, Duration> {
//     match trpl::race(future_to_try, trpl::sleep(max_time)).await {
//         Either::Left(output) => Ok(output),
//         Either::Right(_) => Err(max_time),
//     }

// use trpl::StreamExt;
// use std::time::Duration;

// fn main() {
//     trpl::run(async {
//         let values= 1..101;
//         let iter = values.map(|n| n * 2);
//         let stream = trpl::stream_from_iter(iter);

//         while let Some(value) = 
//             Timeout(Duration::from_secs(1), stream.next()).await.unwrap_or(None) {
//             println!("The value was: {value}");
//         }
//     })
// }

// use trpl::StreamExt;

// fn main() {
//     trpl::run(async {
//         let values = 1..101;
//         let iter = values.map(|n| n * 2);
//         let stream = trpl::stream_from_iter(iter);

//         let mut filtered =
//             stream.filter(|value| value % 3 == 0 || value % 5 == 0);

//         while let Some(value) = filtered.next().await {
//             println!("The value was: {value}");
//         }
//     });
// }

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10)).take(20);
        let merged = messages.merge(intervals);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 500 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")){
                eprintln!("Cannot send message '{message}': {send_error:?}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            
            if let Err(send_error) = tx.send(count) {
                eprintln!("Cannot send interval count {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}