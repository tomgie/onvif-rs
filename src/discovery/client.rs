use crate::discovery::message::{
    build_probe_message, ProbeMatch, ProbeMatchEnvelope, MULTICAST_IPV4_ADDRESS, MULTICAST_PORT,
};
use crate::error::OnvifError;
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;

pub async fn discover(timeout: Duration) -> Result<Vec<ProbeMatch>, OnvifError> {
    let multicast_addr = MULTICAST_IPV4_ADDRESS.parse::<Ipv4Addr>().unwrap();
    let multicast_socket_addr = SocketAddrV4::new(multicast_addr, MULTICAST_PORT);

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;

    let bind_addr = SocketAddrV4::new(Ipv4Addr::from([192, 168, 1, 48]), 0);
    socket.bind(&bind_addr.into())?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(&multicast_addr, &Ipv4Addr::from([192, 168, 1, 48]))?;
    socket.set_nonblocking(true)?;

    let socket: UdpSocket = UdpSocket::from_std(socket.into())?;
    let probe_message = build_probe_message()?;

    socket
        .send_to(probe_message.as_bytes(), multicast_socket_addr)
        .await?;

    let mut devices = Vec::new();
    let mut buf = [0; 4096];

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < timeout {
        let remaining_time = timeout - start_time.elapsed();
        match tokio::time::timeout(remaining_time, socket.recv_from(&mut buf)).await {
            Ok(Ok((len, _addr))) => {
                let response_str = str::from_utf8(&buf[..len]).unwrap_or("");
                if let Ok(envelope) = quick_xml::de::from_str::<ProbeMatchEnvelope>(response_str) {
                    for probe_match in envelope.body.probe_matches.probe_match {
                        if !devices.contains(&probe_match) {
                            devices.push(probe_match);
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Error receiving UDP packet: {}", e);
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok(devices)
}
