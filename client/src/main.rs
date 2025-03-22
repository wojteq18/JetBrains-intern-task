use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::thread;

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

    let host = "127.0.0.1";
    let port = "8080";

    let range_start = 0;
    let range_end = 9999;

    let mut buffer = vec![0 as u8; lenght];

    // Łączenie z serwerem
    let mut connect = connect_to_server(&host, &port)?;
    println!("Połączono z serwerem");
    let request = format!("GET /range/{}-{} HTTP/1.0\r\n\r\n", range_start, range_end);

    //wysyłanie żądania
    connect.write_all(request.as_bytes())?;
    println!("Wysłano żądanie");

    //odbieramy dane
    let mut reader = BufReader::new(connect);
    reader.read_to_end(&mut buffer)?;
    println!("Odebrano dane");
    Ok(())
}