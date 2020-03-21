mod protobuf;

use std::convert::TryInto;
use std::net::{TcpListener};
use std::io::Write;
use protobuf::{BufReader, BufWriter};
use std::ops::Add;

fn main() {

    let bind_ip = "0.0.0.0";
    let port = "25565";

    let mut addr = String::from(bind_ip);
    addr.push_str(":");
    addr.push_str(port);

    let listener = TcpListener::bind(addr).unwrap();

    loop {
        let client = listener.accept().unwrap();
        let mut reader = BufReader::from(client.0);

        let _packet_size = reader.varint();
        let packet_id = reader.byte();

        if packet_id != 0 {
            continue;
        }

        let protocol = reader.varint();

        println!("Ping:");
        println!("Protocol: {}", protocol);

        let cli_addr = reader.string();
        let cli_port = reader.ushort();

        println!("Address: {}:{}", cli_addr, cli_port);

        println!("State: {}",
        match reader.varint() {
            1 => "status",
            2 => "login",
            _ => "unknow"
        });

        println!();

        let mut writer = BufWriter::new();
        writer.byte(0); // PacketID
        let mut description = String::from("{\"description\":{\"text\":\"");
        description.push_str(format!("{}:{}", cli_addr, cli_port).as_str());
        description.push_str("\"},\"players\":{\"max\":20,\"online\":666},\"version\":{\"name\":\"1.15.2\",\"protocol\":578}}");
        writer.string(&description);
        let mut packet = BufWriter::new();
        packet.varint(writer.data.len().try_into().unwrap());
        packet.concat(&mut writer.data);

        reader.get_stream().write_all(packet.data.as_slice()).unwrap();
    }
}
