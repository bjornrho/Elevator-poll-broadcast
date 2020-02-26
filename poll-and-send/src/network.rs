use network_rust::bcast::BcastReceiver;
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use serde::{Serialize, Deserialize};

use network_rust::bcast::BcastTransmitter;
use network_rust::localip;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeStructure {
    pub action: ElevatorActions,
    pub floor: u8,
    pub origin: std::net::IpAddr
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElevatorActions {
    Cabbcall,
    Lobbycall,
    Complete,
}

pub fn start(port: u16, tx_sender: Sender<NodeStructure>, rx_sender: Receiver<NodeStructure>){
    let net_socket = BcastReceiver::new(port).unwrap();
    thread::spawn(move || {
        net_socket.run(tx_sender);
    });
    network_reciver(rx_sender);
}

fn network_reciver(rx_sender: Receiver<NodeStructure>){
    loop {
        let message = rx_sender.recv().unwrap();
        println!("Got something: {:?} {:?}", message.action, message.floor);
    }
}

pub fn broadcast_button_push(buttontype: ElevatorActions, floor: u8){
    let call: ElevatorActions;
    match buttontype {
        Cabbcall => {
            call = ElevatorActions::Lobbycall;
        },
        Lobbycall => {
            call = ElevatorActions::Cabbcall;
        },
        Complete => {call = ElevatorActions::Complete}
    };
    let (tx, rx) = channel::<NodeStructure>();
    let network = thread::spawn(move || {
        start(50500, tx, rx);
    });
    let broadcast = BcastTransmitter::new(50500).unwrap();
    let msg = NodeStructure {
        action: call,
        floor: floor,
        origin: localip::get_localip().unwrap(),
    };
    for _ in 0..10 {
        broadcast.transmit(&msg).unwrap();
    }
}