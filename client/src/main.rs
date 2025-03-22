use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::thread;
use sha2::{Sha256, Digest};

fn connect_to_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let addr = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(addr)?;
    Ok((stream))
}


fn get_lenght() -> usize {
    println!("Enter expected length: ");
    let mut lenght = String::new();
    std::io::stdin().read_line(&mut lenght).unwrap();
    let lenght: usize = lenght.trim().parse().unwrap();
    lenght
}
fn main() -> std::io::Result<()> {
    let lenght = get_lenght();
    let mut i = 0;
    let host = "127.0.0.1";
    let port = "8080";

    let mut buffer: Vec<u8> = Vec::new();

    while buffer.len() < lenght {
        let mut connect = connect_to_server(&host, &port)?;
        let request = format!(
            "GET / HTTP/1.1\r\n\
             Host: {host}:{port}\r\n\
             Range: bytes={}-{}\r\n\
             Connection: close\r\n\
             \r\n",
            buffer.len(),
            buffer.len() + 9999
        );        

        //wysyłanie żądania
        connect.write_all(request.as_bytes())?;
        println!("Wysłano żądanie {}", i);

        //odbieramy dane
        let mut reader = BufReader::new(connect);
        reader.read_to_end(&mut buffer)?;
        i = i + 1;
        println!("Obecnie: {}", buffer.len());
        }
        println!("Everything allright"); 
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let result = hasher.finalize();
        println!("{:x}", result);

    Ok(())
}