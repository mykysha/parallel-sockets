use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn f(x: i32) -> i32 {
    // Replace this function body with the desired implementation
    x + 1
}

fn g(x: i32) -> i32 {
    // Replace this function body with the desired implementation
    x * 2
}

fn handle_client(mut stream: TcpStream, func: fn(i32) -> i32, input: i32) {
    let result = func(input);
    stream.write_all(&result.to_ne_bytes()).unwrap();
}

fn main() {
    let f_listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let g_listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    println!("Enter a value for x: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();

    let f_thread = thread::spawn(move || {
        for stream in f_listener.incoming() {
            let stream = stream.unwrap();
            handle_client(stream, f, x);
        }
    });

    let g_thread = thread::spawn(move || {
        for stream in g_listener.incoming() {
            let stream = stream.unwrap();
            handle_client(stream, g, x);
        }
    });

    let mut f_result = TcpStream::connect("127.0.0.1:8080");
    let mut g_result = TcpStream::connect("127.0.0.1:8081");

    loop {
        match f_result.as_ref() {
            Ok(f_stream) => {
                let mut cloned_stream = f_stream.try_clone().unwrap();
                let mut buf = [0; 4];
                cloned_stream.read_exact(&mut buf).unwrap();
                let f_res: i32 = i32::from_ne_bytes(buf);

                if f_res != 0 {
                    println!("Result: {}", f_res);
                    break;
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(10));
            }
        }

        match g_result.as_ref() {
            Ok(g_stream) => {
                let mut cloned_stream = g_stream.try_clone().unwrap();
                let mut buf = [0; 4];
                cloned_stream.read_exact(&mut buf).unwrap();
                let g_res: i32 = i32::from_ne_bytes(buf);

                if g_res != 0 {
                    println!("Result: {}", g_res);
                    break;
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_secs(10));
            }
        }

        println!("1) Continue the calculation");
        println!("2) Stop");
        println!("3) Continue without asking more");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                thread::sleep(Duration::from_secs(10));
            }
            2 => {
                break;
            }
            3 => {
                loop {
                    match f_result {
                        Ok(ref mut f_stream) => {
                            let mut buf = [0; 4];
                            f_stream.read_exact(&mut buf).unwrap();
                            let f_res: i32 = i32::from_ne_bytes(buf);

                            if f_res != 0 {
                                println!("Result: {}", f_res);
                                break;
                            }
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_secs(10));
                        }
                    }

                    match g_result {
                        Ok(ref mut g_stream) => {
                            let mut buf = [0; 4];
                            g_stream.read_exact(&mut buf).unwrap();
                            let g_res: i32 = i32::from_ne_bytes(buf);

                            if g_res != 0 {
                                println!("Result: {}", g_res);
                                break;
                            }
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_secs(10));
                        }
                    }
                }
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }

    f_thread.join().unwrap();
    g_thread.join().unwrap();
}

