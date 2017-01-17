# rust-bananapi-dht11

An experiment to implement a [DHT11](https://www.adafruit.com/product/386) (temperature/humidity sensor) driver on a [Banana Pi](https://en.wikipedia.org/wiki/Banana_Pi) using [Rust](https://www.rust-lang.org/en-US/).


## Wiring

After the resistor VCC/GND setup, communication is done through an GPIO pin.

![DHT11/Banana Pi wiring](https://alexbloggt.com/wp-content/uploads/2014/12/bananapi_dht11_pullup.png)
Image [source](https://alexbloggt.com/dht11-banana-pi-teil1/)


## DHT11 Protocol

The DHT11 uses the same pin for Input and Output. The protocols starts by the controller signalling the start of the reading. Then the communication flow inverts, with the DHT11 sending some sort of header, followed by the readings (humidity and then temperature). Bits are indicate by the time the pin is left high or low. More details can be found [here](http://www.candrian.gr/index.php/dht-11-one-wire-bus/).


## Implementation

The implementation is implemented in Rust and is quite naive, in the sense that nothing like interrupts or timers are used. When writing, it just updates pin values and sleeps. On reading, it polls the pin level and checks timestamps. It means that if the Linux scheduler doesn't cooperates, the timing will be compromised, and reading will fail.

The Banana Pi is running the latest version of [Bananian](http://www.lemaker.org/product-bananapi-download-9.html), a Debian-based distro tailored for it.

The implementation depends on the Banana Pi's [GPIO library](http://wiki.lemaker.org/BananaPro/Pi:GPIO_library), available [here](https://github.com/LeMaker/WiringBP/tree/bananapi).

**Pin #1 is being used**.


## Compilation and Running

Instead of installing the whole toolchain on my machine, I'm using a pre-backed Docker image: `rustcross/armv7-unknown-linux-gnueabihf`.

To get started, you can launch an instance of the image with:

```bash
cd "$project"
docker run --rm -v "$PWD:/work" rustcross/armv7-unknown-linux-gnueabihf:latest bash
```

Once there, you first need to compile the GPIO library:

```bash
cd
git clone https://github.com/LeMaker/WiringBP -b bananapi
cd WiringBP/wiringPi
make CC=arm-linux-gnueabihf-gcc
make install
```

Afterwards, you can build the project using Cargo:

```bash
cd /work
cargo build --target=armv7-unknown-linux-gnueabihf --release
```

This generates a binary on the target dir (`target/armv7-unknown-linux-gnueabihf/release/dht11`) that can be copied and run inside the Banana Pi.
