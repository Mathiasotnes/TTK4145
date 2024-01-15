use std::net::{UdpSocket, TcpListener};
use std::io::{Result, Read, Write};

fn main() -> Result<()> {
    tcp_test()
}

fn udp_test() -> Result<()> {

    /* Create socket */
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    /* Receive data */
    let mut buf = [0; 20];
    let (amt, src) = socket.recv_from(&mut buf)?;

    /* Print data */
    let received_data = std::str::from_utf8(&buf[..amt]).expect("Could not write buffer as string");
    println!("Packet received from: {}", src.to_string());
    println!("Received data: {}", received_data);

    /* Reverse packet */
    let reversed_data = &mut buf[..amt];
    reversed_data.reverse();
    socket.send_to(reversed_data, &src)?;
    println!("Packet sent back to: {}", src.to_string());
    println!("Sent data: {}", std::str::from_utf8(&reversed_data).expect("Could not write buffer as string"));

    Ok(())
}

fn tcp_test() -> Result<()> {

    /* Create socket */
    let listener = TcpListener::bind("127.0.0.1:34254")?;

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

/* Commands for testing on MacOS using netcat */

/* UDP */
/* echo 'Hello, world!' | nc -u 127.0.0.1 34254 */

/* TCP */
/* nc 127.0.0.1 34254 */
/* Hello, world! */

/* For documentation, check out:
 * https://doc.rust-lang.org/std/net/struct.UdpSocket.html 
 */