#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, StackResources};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use cyw43_pio::PioSpi;
use embassy_net::tcp::TcpSocket;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Starting Pico 2 W Wi-Fi demo...");

    // Initialize Wi-Fi SPI driver
    let pio = Pio::new(PIO0, Irqs);
    let mut wifi_spi = PioSpi::new(pio, DMA_CH0);
    let cyw43 = cyw43::new(wifi_spi).await.unwrap();

    // DHCP config with hostname
    let mut dhcp = DhcpConfig::default();
    dhcp.hostname = Some("pico2w-rust".try_into().unwrap());
    let config = Config::dhcpv4(dhcp);

    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let stack = embassy_net::Stack::new(cyw43, config, RESOURCES.init(StackResources::new()));

    stack.run().await;

    info!("Connecting to Wi-Fi...");
    stack.join("YourSSID", "YourPassword").await.unwrap();

    info!("Connected! Fetching example.com...");
    let mut socket = TcpSocket::new(&stack);
    socket.connect("93.184.216.34:80").await.unwrap(); // example.com

    let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
    socket.write_all(request).await.unwrap();

    let mut buffer = [0u8; 1024];
    let n = socket.read(&mut buffer).await.unwrap();
    info!("Received: {}", core::str::from_utf8(&buffer[..n]).unwrap());

    loop {
        Timer::after(Duration::from_secs(5)).await;
        info!("Still alive...");
    }
}
