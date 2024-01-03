
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { // need to modify your server to handle each connection in a separate task. You can do this using the tokio::spawn function, which runs a future on the Tokio runtime's thread pool.
            handle_connection(stream).await;
        });
    }
    
}

async fn handle_connection(mut stream: TcpStream) {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    // connection was closed by the client
                    println!("Connection closed: {}", stream.peer_addr().unwrap());
                    break;
                }
                println!("Received command: {}", String::from_utf8_lossy(&buffer[..size]));
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).await.unwrap();
                stream.flush().await.unwrap();
                println!("Sent response: {}", response);
            },
            Err(e) => {
                // Handle any error that might occur during read
                println!("Error reading from connection: {}", e);
                break;
            },
        }
    }
}

// important notes
// The tokio::spawn function spawns a new asynchronous task on the Tokio runtime. This task is then executed concurrently with other tasks on the runtime.
// However, it's important to note that "concurrent" does not necessarily mean "in parallel" or "on a different thread". The Tokio runtime uses a thread pool to execute tasks, and it uses an event-driven model with non-blocking I/O to achieve high concurrency even on a small number of threads.
// When you spawn a task with tokio::spawn, it gets scheduled to run on one of the threads in the Tokio runtime's thread pool. It might run on the same thread as the task that called spawn, or it might run on a different thread, depending on how the runtime schedules tasks.
// In other words, tokio::spawn allows for concurrent execution of tasks, but whether or not those tasks run in parallel on different threads is an implementation detail of the Tokio runtime.