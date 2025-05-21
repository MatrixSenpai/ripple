use assign_resources::assign_resources;
use embassy_rp::{bind_interrupts, i2c, peripherals, pio};

bind_interrupts!(
    pub struct Irqs {
        I2C0_IRQ => i2c::InterruptHandler<peripherals::I2C0>;
        PIO0_IRQ_0 => pio::InterruptHandler<peripherals::PIO0>;
    }
);

assign_resources! {
    mpl115a2: MplResources {
        controller: I2C0,
        scl: PIN_17,
        sda: PIN_16,
    },
    dht11: DhtResources {
        pin: PIN_14,
    },
    sh1106: ShResources {
        controller: SPI0,
        clk: PIN_2,
        miso: PIN_3,
        dc: PIN_4,
        cs: PIN_5,
        rst: PIN_6,
    },
    ws2812: WsResources {
        controller: PIO0,
        dma: DMA_CH0,
        data: PIN_21
    }
}

