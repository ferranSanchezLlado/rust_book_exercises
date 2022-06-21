/// Let’s make the following changes to what happens when we create a ThreadPool. We’ll implement
/// the code that sends the closure to the thread after we have Worker set up in this way:
///
///     1. Define a Worker struct that holds an id and a JoinHandle<()>.
///     2. Change ThreadPool to hold a vector of Worker instances.
///     3. Define a Worker::new function that takes an id number and returns a Worker instance that
///         holds the id and a thread spawned with an empty closure.
///     4. In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with
///         that id, and store the worker in the vector.
///
/// If you’re up for a challenge, try implementing these changes on your own before looking at the
/// code in Listing 20-15.
#[allow(dead_code)]
#[allow(unused_variables)]
pub mod single_to_multithreading {
    pub mod thread_pool {
        use std::thread;

        struct Worker {
            id: usize,
            thread: thread::JoinHandle<()>,
        }

        impl Worker {
            pub fn new(id: usize) -> Self {
                let thread = thread::spawn(|| {});
                Worker { id, thread }
            }
        }

        pub struct ThreadPool {
            threads: Vec<Worker>,
        }

        impl ThreadPool {
            /// Create a new ThreadPool.
            ///
            /// The size is the number of threads in the pool.
            ///
            /// # Panics
            ///
            /// The `new` function will panic if the size is zero.
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let mut threads = Vec::with_capacity(size);

                for id in 0..size {
                    threads.push(Worker::new(id));
                }

                ThreadPool { threads }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }
    }

    pub mod web_server {
        use super::thread_pool::ThreadPool;
        use std::fs;
        use std::io::prelude::*;
        use std::net::TcpListener;
        use std::net::TcpStream;
        use std::thread;
        use std::time::Duration;

        pub fn run_server() {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            let pool = ThreadPool::new(4);

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let get = b"GET / HTTP/1.1\r\n";
            let sleep = b"GET /sleep HTTP/1.1\r\n";

            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK", "hello.html")
            } else if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "404.html")
            };

            let contents = fs::read_to_string(filename).unwrap();

            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

/// We could do more here! If you want to continue enhancing this project, here are some ideas:
///
///     - Add more documentation to ThreadPool and its public methods.
///     - Add tests of the library’s functionality.
///     - Change calls to unwrap to more robust error handling.
///     - Use ThreadPool to perform some task other than serving web requests.
///     - Find a thread pool crate on crates.io and implement a similar web server using the crate
///         instead. Then compare its API and robustness to the thread pool we implemented.
#[allow(dead_code)]
pub mod final_project {

    /// Module implementing a basic [ThreadPool] that can be used to execute multiple tasks with a
    /// limited amount of threads. The ThreadPool is implemented using a [Vec] of internal [Worker]
    /// structs that are spawned by the ThreadPool. The ThreadPool is responsible for spawning and
    /// joining the [Worker] threads. The communication between the ThreadPool and the [Worker]
    /// threads is done using a [Sender] and a [Receiver].
    ///
    /// [Sender]: std::sync::mpsc::Sender
    /// [Receiver]: std::sync::mpsc::Receiver
    pub mod thread_pool {
        use std::sync::{mpsc, Arc, Mutex};
        use std::thread;

        /// Basic Implementation of a Thread Pool, allowing the consumption of unlimited tasks with
        /// a fixed amount of threads. Should be able to solve the same tasks as spawning a new
        /// thread for each new task.
        ///
        /// The `ThreadPool` accepts closures with no inputs or outputs.
        ///
        /// If the `ThreadPool` is dropped, all threads will receive a new message indicating to
        /// terminate execution and the threads will be joined. This causes that if the task never
        /// finishes, the threads will never be joined halting the program.
        ///
        /// # Example
        /// ```rust
        /// let pool = ThreadPool::new(8);
        ///
        /// for i in 0..100 {
        ///     pool.execute(move || {
        ///         println!("{}", i);
        ///     });
        /// }
        /// ```
        ///
        /// # Panics
        ///
        /// The ThreadPool needs at least one thread.
        pub struct ThreadPool {
            workers: Vec<Worker>,
            sender: mpsc::Sender<Message>,
        }

        type Job = Box<dyn FnOnce() + Send + 'static>;

        enum Message {
            NewJob(Job),
            Terminate,
        }

        impl ThreadPool {
            /// Create a new ThreadPool.
            ///
            /// The size is the number of threads in the pool.
            ///
            /// # Panics
            ///
            /// The `new` function will panic if the size is zero.
            pub fn new(size: usize) -> ThreadPool {
                if size == 0 {
                    panic!("ThreadPool size must be greater than zero.");
                }

                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                ThreadPool { workers, sender }
            }

            /// Adds a new task to be executed by one of the threads in the pool. If any thread is
            /// available, the task will be executed immediately. Otherwise, the task will not be
            /// executed until one of the threads finishes the task it was working.
            ///
            /// # Panics
            ///
            /// The `execute` function will panic if the message could not be able to send the job
            /// to the receivers pool.
            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                let job = Box::new(f);

                self.sender.send(Message::NewJob(job))
                    .expect("ThreadPool::execute unable to send job into queue.");
            }
        }

        impl Drop for ThreadPool {
            fn drop(&mut self) {
                println!("Sending terminate message to all workers.");

                for _ in &self.workers {
                    self.sender.send(Message::Terminate).unwrap();
                }

                println!("Shutting down all workers.");

                for worker in &mut self.workers {
                    println!("Shutting down worker {}", worker.id);

                    if let Some(thread) = worker.thread.take() {
                        thread.join().unwrap();
                    }
                }
            }
        }

        struct Worker {
            id: usize,
            thread: Option<thread::JoinHandle<()>>,
        }

        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
                let thread = thread::spawn(move || loop {
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message {
                        Message::NewJob(job) => {
                            // println!("Worker {} got a job; executing.", id);

                            job();
                        }
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);

                            break;
                        }
                    }
                });

                Worker {
                    id,
                    thread: Some(thread),
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            fn basic_test(size: usize) {
                let pool = ThreadPool::new(size);

                let counter = Arc::new(Mutex::new(0));
                for _ in 0..10 {
                    let counter = Arc::clone(&counter);
                    pool.execute(move || {
                        let mut guard = counter.lock().unwrap();
                        *guard += 1;
                    })
                }
                drop(pool);
                assert_eq!(*counter.lock().unwrap(), 10);
            }

            #[test]
            fn single_thread() {
                basic_test(1);
            }

            #[test]
            fn multiple_threads() {
                basic_test(8);
            }

            /// Test is failing because the threads are not being joined if the function is an
            /// infinite loop.
            #[test]
            #[ignore]
            fn drop_on_infinite_loop() {
                let pool = ThreadPool::new(1);
                pool.execute(|| loop {});

                // This will cause the thread to never join.
                drop(pool); // Not needed as the test ends here.
            }

            #[test]
            #[should_panic]
            fn zero_threads() {
                ThreadPool::new(0);
            }
        }
    }

    /// Basic implementation of a web server capable of handling multiple clients at the same time
    /// without the risk of DOS. The website has two valid roots:
    ///
    /// - `/`: Shows a static HTML webpage located on `./html/hello.html`.
    /// - `/sleep`: First sleeps the thread for two seconds and displays the same website as root (`\`).
    /// - `others`: Displays an error HTML website located on `./html/404.html`.
    ///
    /// # Example
    ///
    /// To run the start the server run the following command on the `main()` function.
    ///
    /// ```rust
    /// run_server()
    /// ```
    pub mod web_server {
        use std::fs;
        use std::io::prelude::*;
        use std::net::TcpListener;
        use std::net::TcpStream;
        use std::thread;
        use std::time::Duration;
        use threadpool::ThreadPool;

        /// Function is renamed from main.rs
        pub fn run_server() {
            let listener =
                TcpListener::bind("127.0.0.1:7878").expect("Could not bind to port: 7878");
            let pool = ThreadPool::new(4);

            for stream in listener.incoming() {
                let stream = match stream {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprintln!("Stream error: {}", e);
                        continue;
                    }
                };

                pool.execute(|| {
                    handle_connection(stream);
                })
            }

            println!("Shutting down.");
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            if let Err(err) = stream.read(&mut buffer) {
                eprintln!("Error reading from stream: {}", err);
                return;
            }

            let get = b"GET / HTTP/1.1\r\n";
            let sleep = b"GET /sleep HTTP/1.1\r\n";

            let (status_line, filename) = if buffer.starts_with(get) {
                ("HTTP/1.1 200 OK", "html/hello.html")
            } else if buffer.starts_with(sleep) {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "html/hello.html")
            } else {
                ("HTTP/1.1 404 NOT FOUND", "html/404.html")
            };

            let contents = fs::read_to_string(filename)
                .expect(format!("HTML file not found: {}", filename).as_str());

            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            );

            if let Err(err) = stream.write(response.as_bytes()) {
                eprintln!("Error writing to stream: {}", err);
                return;
            }
            if let Err(err) = stream.flush() {
                eprintln!("Error flushing stream: {}", err);
                return;
            }
        }
    }

    /// Module which uses a thread pool to calculate PI. A [ThreadPool] is used to handle the
    /// calculation of the PI. The operation is divided into a number of iterations which are then
    /// joined together to form a single value.
    ///
    /// The calculation is done using the following integral (0 to 1):
    /// ```text
    /// pi = 4 / 1+x^2 dx
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// let pi = pi::calculate_pi(8, 1_000_000);
    /// println!("Pi: {}", pi);
    /// ```
    pub mod pi {
        use super::thread_pool::ThreadPool;
        use std::sync::{Arc, Mutex};

        /// Calculates the number pi by using the following integral (0 to 1):
        /// ```text
        /// pi = 4 / 1+x^2 dx
        /// ```
        ///
        /// # Example
        ///
        /// ```rust
        /// let pi = pi::calculate_pi(8, 1_000_000);
        /// println!("Pi: {}", pi);
        /// ```
        ///
        /// # Panics
        /// This function will panic if the number of threads is less than or equal to 0. More
        /// information can be found in the [`ThreadPool`] module.
        pub fn calculate_pi(num_threads: usize, iterations: usize) -> f64 {
            let pool = ThreadPool::new(num_threads);
            let pi = Arc::new(Mutex::new(0.0));

            for id in 0..iterations {
                let pi = Arc::clone(&pi);
                pool.execute(move || {
                    let value = integrate(id, iterations);

                    let mut guard = pi.lock().unwrap();
                    *guard += value;
                })
            }
            drop(pool);
            let pi = *pi.lock().unwrap();
            pi
        }

        fn integrate(iteration: usize, max_iterations: usize) -> f64 {
            let width = 1.0 / (max_iterations as f64);
            let mid = (iteration as f64 + 0.5) * width;
            let height = 4.0 / (1.0 + mid * mid);
            height * width
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use std::f64::consts::PI;

            fn check_difference(pi: f64, max_difference: f64) {
                let diff = PI - pi;

                if diff.abs() > max_difference {
                    panic!("Calculated PI is too different from real value: {}", diff);
                }
            }

            #[test]
            fn basic_test() {
                let pi = calculate_pi(1, 100);
                check_difference(pi, 1e-5);
            }

            #[test]
            fn multiple_threads() {
                let pi = calculate_pi(8, 100);
                check_difference(pi, 1e-5);
            }

            #[test]
            fn large_iterations_and_threads() {
                let pi = calculate_pi(8, 1_000_000);
                check_difference(pi, 1e-12);
            }
        }
    }
}
