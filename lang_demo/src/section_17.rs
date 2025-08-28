///////////////////////////////
// Course Section 17
///////////////////////////////

// Asnyc version of std libraries
use async_std::{ fs::File, io, prelude::*, task };

// Async read from file
// An async function has the performance advantage in that it allows the 
// runtime to do other activities while it waits for I/O completion.
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

pub fn run_lesson() {
    println!("\nSection 17:");

    // Creats async code block
    let read_task = task::spawn(async {
        let res = read_file("test_17.txt").await;
        match res {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    });

    // Invoke the async task
    println!("Task has started");
    task::block_on(read_task);
    println!("Task has stopped");
}
