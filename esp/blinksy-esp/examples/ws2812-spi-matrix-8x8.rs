#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use blinksy::driver::ClocklessSpiBuilder;
use blinksy::{
    driver::{
        clockless_spi_buffer_size, clockless_spi_ideal_frequency_hz, clockless_spi_pulse_size,
        ClocklessDriver,
    },
    layout::{Layout2d, Shape2d, Vec2},
    layout2d,
    leds::Ws2812,
    patterns::rainbow::{Rainbow, RainbowParams},
    ControlBuilder,
};
use blinksy_esp::time::elapsed;
use embassy_executor::Spawner;
use esp_alloc as _;
use esp_hal::{
    dma_rx_buffer, dma_tx_buffer,
    spi::master::{Config, Spi, SpiDma},
    time::Rate,
    timer::timg::TimerGroup,
    Async,
};
use esp_println as _;

extern crate alloc;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let cpu_clock = esp_hal::clock::CpuClock::max();
    let config = esp_hal::Config::default().with_cpu_clock(cpu_clock);
    let p = esp_hal::init(config);

    let timg0 = TimerGroup::new(p.TIMG0);
    esp_rtos::start(timg0.timer0, p.FROM_CPU_INTR0);

    layout2d!(
        Layout,
        [Shape2d::Grid {
            start: Vec2::new(1., -1.),
            horizontal_end: Vec2::new(-1., -1.),
            vertical_end: Vec2::new(1., 1.),
            horizontal_pixel_count: 8,
            vertical_pixel_count: 8,
            serpentine: false
        }]
    );

    let dma_rx_buf = dma_rx_buffer!(1024).unwrap();
    let dma_tx_buf = dma_tx_buffer!(1024).unwrap();

    // let's see what an ideal SPI frequency would be
    const SPI_FREQ_HZ: u32 = clockless_spi_ideal_frequency_hz::<Led>(150);

    type Led = Ws2812;
    let spi = Spi::new(
        p.SPI2,
        Config::default().with_frequency(Rate::from_hz(SPI_FREQ_HZ)),
    )
    .unwrap()
    .with_mosi(p.GPIO17)
    .with_dma(p.DMA_CH0)
    .with_buffers(dma_rx_buf, dma_tx_buf)
    .into_async();
    let writer = ClocklessSpiBuilder::default()
       .with_spi(spi)
       .with_freq_hz(SPI_FREQ_HZ)
       .with_buffer_size::<{ clockless_spi_buffer_size::<Led, SpiDma<Async>, _>(Layout::PIXEL_COUNT, SPI_FREQ_HZ) }>()
       .with_pulse_size::<{ clockless_spi_pulse_size::<Led, SpiDma<Async>, _>(SPI_FREQ_HZ) }>()
       .with_async()
       .build();

    defmt::info!(
        "ideal freq: {} Hz, error: {} ns, duty cycle bits: {}",
        SPI_FREQ_HZ,
        writer.timing.max_error_ns(),
        writer.timing.duty_cycle_bits()
    );

    let driver = ClocklessDriver::default()
        .with_led::<Led>()
        .with_writer(writer);

    let mut control = ControlBuilder::new_2d_async()
        .with_layout::<Layout, { Layout::PIXEL_COUNT }>()
        .with_pattern::<Rainbow>(RainbowParams {
            ..Default::default()
        })
        .with_driver(driver)
        .with_frame_buffer_size::<{ Led::frame_buffer_size(Layout::PIXEL_COUNT) }>()
        .build();
    control.set_brightness(0.1); // Set initial brightness (0.0 to 1.0)

    loop {
        let elapsed_in_ms = elapsed().as_millis();
        control.tick(elapsed_in_ms).await.unwrap();
    }
}
