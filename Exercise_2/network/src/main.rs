use std::net::{UdpSocket, TcpListener, TcpStream};
use std::io::{Result, Read, Write};

fn main() -> Result<()> {
    udp_send();
    udp_recv()
    // tcp_connect()
}

fn udp_recv() -> Result<()> {

    /* Create socket */
    let socket = UdpSocket::bind("0.0.0.0:20022")?;

    /* Send packet */
    // socket.send_to(b"Hej hej", "10.100.23.1:30000")?;

    /* Receive data */
    let mut buf = [0; 50];
    let (amt, src) = socket.recv_from(&mut buf)?;

    /* Print data */
    let received_data = std::str::from_utf8(&buf[..amt]).expect("Could not write buffer as string");
    println!("Packet received from: {}", src.to_string());
    println!("Received data: {}", received_data);

    Ok(())
}

fn udp_send() -> Result<()> {

    /* Create socket */
    let socket = UdpSocket::bind("0.0.0.0:20022")?;

    /* Send packet */
    socket.send_to(b"Hejhej:)", "10.100.23.129:20022")?;

    Ok(())
}

fn tcp_test() -> Result<()> {

    /* Create socket */
    let listener = TcpListener::bind("0.0.0.0:34933")?;

    /* Accept a single connection */
    let (mut stream, addr) = listener.accept()?;
    println!("Connection established with {}", addr);

    /* Buffer to store the received data */
    let mut buf = [0; 20];

    /* Read data from the stream */
    let amt = stream.read(&mut buf)?;
    let received_data = std::str::from_utf8(&buf[..amt])
        .expect("Could not write buffer as string");
    println!("Received data: {}", received_data);

    /* Reverse the data */
    let reversed_data = &mut buf[..amt];
    reversed_data.reverse();
    
    /* Send the reversed data back */
    stream.write_all(reversed_data)?;
    println!("Sent data: {}", std::str::from_utf8(reversed_data).expect("Could not write buffer as string"));

    Ok(())
}

fn tcp_connect() -> Result<()> {

    let mut send_buf = [0; 1024];

    /* Create socket */
    let listener = TcpListener::bind("10.100.23.32:20093")?;
    let mut send_stream = TcpStream::connect("10.100.23.129:33546")?;

    send_stream.write(b"Connect to: 10.100.23.32:20093\0")?;

    send_stream.read(&mut send_buf)?;
    println!("Received on 33546: \n\r{}", std::str::from_utf8(&send_buf).unwrap());

    /* Accept a single connection */
    let (mut recv_stream, addr) = listener.accept()?;
    println!("Connection established with {}:\n\r", addr);

    send_stream.write(b"U dog\0")?;
    send_stream.write(b"Woof\0")?;
    send_stream.write(b"Woof\0")?;

    /* Buffer to store the received data */
    let mut recv_buf = [0; 1024];

    /* Read data from the stream */
    let amt = recv_stream.read(&mut recv_buf)?;
    let received_data = std::str::from_utf8(&recv_buf[..amt])
        .expect("Could not write buffer as string");
    println!("Received data on : {}:\n\r", received_data);

    Ok(())
} 

/* Commands for testing on MacOS using netcat */

/* UDP */
/* echo 'Hello, world!' | nc -u 127.0.0.1 34254 */

/* TCP */
/* nc 127.0.0.1 34254 */
/* Hello, world! */

/* For documentation, check out:
 * https://doc.rust-lang.org/std/net/struct.UdpSocket.html 
 */