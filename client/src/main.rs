use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use sha2::{Sha256, Digest};

fn connect_to_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr)?;
    Ok(stream)
}


fn get_lenght() -> usize {
    println!("Enter expected length: ");
    let mut lenght = String::new();
    std::io::stdin().read_line(&mut lenght).unwrap();
    let lenght: usize = lenght.trim().parse().unwrap();
    return lenght
}

fn main() -> std::io::Result<()> {
    let lenght = get_lenght();
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

        connect.write_all(request.as_bytes())?;

        let mut reader = BufReader::new(connect);
        let mut response = Vec::new();
        reader.read_to_end(&mut response)?;

        match response.windows(4).position(|w| w == b"\r\n\r\n") {
            Some(pos) => {
                let body_start = pos + 4;
                let body = &response[body_start..];
                buffer.extend_from_slice(body);
            },
            None => {
                eprintln!("No end of headers found");
                break;
            }
        }
    }

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();
    println!("{:x}", result);

    Ok(())
}