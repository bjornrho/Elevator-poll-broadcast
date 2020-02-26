use std::sync::mpsc::channel;
use std::string;
use std::thread;
use std::time;
// mod elev_driver;

use elevator_driver::elev_driver::net_io::*;
use elevator_driver::elev_driver::*;

use network_rust::bcast::BcastTransmitter;
use network_rust::localip;

mod network;




fn main() {
    let driver = ElevIo::new().unwrap();
    
    const SEC_TOP: u8 = N_FLOORS;
    loop {
        for floor in 0..N_FLOORS {
            match driver.get_button_signal(Button::Internal(Floor::At(floor))).expect("Button signal error") {
                Signal::High => {
                    network::broadcast_button_push(network::ElevatorActions::Cabbcall, floor);
                    loop {
                        match driver.get_floor_signal()
                                    .expect("Get FloorSignal failed") {
                            Floor::At(data) => {
                                if data > floor{
                                    driver.set_motor_dir(MotorDir::Down).expect("Set MotorDir failed");
                                }
                                if data < floor{
                                    driver.set_motor_dir(MotorDir::Up).expect("Set MotorDir failed");
                                }
                                if data == floor{
                                    driver.set_motor_dir(MotorDir::Stop).expect("Set MotorDir failed");
                                    break;
                                }
                            }
                            Floor::Between => {

                            }
                        }
                    }
                }
                Signal::Low => {

                }
            }
        }
        for floor in 0..N_FLOORS-1 {
            match driver.get_button_signal(Button::CallUp(Floor::At(floor))).expect("Button signal error") {
                Signal::High => {
                    network::broadcast_button_push(network::ElevatorActions::Lobbycall, floor);
                    loop {
                        match driver.get_floor_signal()
                                    .expect("Get FloorSignal failed") {
                            Floor::At(data) => {
                                if data > floor{
                                    driver.set_motor_dir(MotorDir::Down).expect("Set MotorDir failed");
                                }
                                if data < floor{
                                    driver.set_motor_dir(MotorDir::Up).expect("Set MotorDir failed");
                                }
                                if data == floor{
                                    driver.set_motor_dir(MotorDir::Stop).expect("Set MotorDir failed");
                                    break;
                                }
                            }
                            Floor::Between => {

                            }
                        }
                    }
                }
                Signal::Low => {

                }
            }
        }
        for floor in 1..N_FLOORS {
            match driver.get_button_signal(Button::CallDown(Floor::At(floor))).expect("Button signal error") {
                Signal::High => {
                    network::broadcast_button_push(network::ElevatorActions::Lobbycall, floor);
                    loop {
                        match driver.get_floor_signal()
                                    .expect("Get FloorSignal failed") {
                            Floor::At(data) => {
                                if data > floor{
                                    driver.set_motor_dir(MotorDir::Down).expect("Set MotorDir failed");
                                }
                                if data < floor{
                                    driver.set_motor_dir(MotorDir::Up).expect("Set MotorDir failed");
                                }
                                if data == floor{
                                    driver.set_motor_dir(MotorDir::Stop).expect("Set MotorDir failed");
                                    break;
                                }
                            }
                            Floor::Between => {

                            }
                        }
                    }
                }
                Signal::Low => {

                }
            }
        }

        if let Signal::High = driver.get_stop_signal().expect("Get StopSignal failed") {
            driver.set_motor_dir(MotorDir::Stop)
                .expect("Set MotorDir failed");
            return;
        }
        
    }

    driver.io.lifeline.join();
    /*
    let (sending_tx, sending_rx) = channel::<std::vec::Vec<u8>>();
    let (reciving_tx, reciving_rx) = channel::<std::vec::Vec<u8>>();
    let net = thread::spawn(move || {
        let mut network = Communication::new("localhost".to_string(), 15657, sending_rx, reciving_tx).unwrap();
        network.start();
    });
    sending_tx.send(vec![6,0,0,1]).unwrap();
    loop {
        println!("Got something! {:?}", reciving_rx.recv().unwrap())
    }
    net.join();
    */
}