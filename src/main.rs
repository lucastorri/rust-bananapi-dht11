use std::{thread, time, process};

extern crate libc;
use libc::{c_int};

#[link(name = "wiringPi")]
extern {
  fn wiringPiSetup();
  fn pinMode(pin: c_int, value: c_int);
  fn digitalWrite(pin: c_int, value: c_int);
  fn digitalRead(pin: c_int) -> c_int;
}

static PIN: c_int = 1;

fn main() {

  unsafe { wiringPiSetup() };

  // MCU Request
  unsafe { pinMode(PIN, 1) };
  send_and_wait(1);
  send_and_wait(0);

  // DHT11 Response Header
  unsafe { pinMode(PIN, 0) };
  busy_wait_bit();

  // DHT11 Response Data
  let mut response: [i8; 5] = [0; 5];
  let threshold = time::Duration::new(0, 40000);
  for elem in response.iter_mut() {
    for j in (0..8).rev() {
      if busy_wait_bit() > threshold {
        *elem |= 1 << j;
      }
    }
  }


  // Validate and Print
  if !is_valid(&response) {
    process::exit(1)
  }

  println!("h%: {}", response[0]);
  println!("tC: {}", response[2]);
}

fn is_valid(response: &[i8]) -> bool {
  response.iter().fold(0, |a, &b| a + b) == (response.last().unwrap_or(&0) * 2)
}

fn send(v: c_int) {
  unsafe { digitalWrite(PIN, v) };
}

fn receive() -> c_int {
  unsafe { digitalRead(PIN) }
}

fn send_and_wait(v: c_int) {
  send(v);
  thread::sleep(time::Duration::from_millis(30));
}

fn busy_wait_bit() -> time::Duration {
  busy_wait_start(0);
  busy_wait_finish(0);
  let start = time::Instant::now();
  busy_wait_finish(1);
  time::Instant::now().duration_since(start)
}

fn busy_wait_start(v: c_int) {
  while receive() != v {
    thread::sleep(time::Duration::new(0, 1000));
  }
}

fn busy_wait_finish(v: c_int) {
  while receive() == v {
    thread::sleep(time::Duration::new(0, 1000));
  }
}
