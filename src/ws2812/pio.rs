use embassy_rp::dma::{AnyChannel, Channel};
use embassy_rp::{into_ref, Peripheral, PeripheralRef};
use embassy_rp::clocks::clk_sys_freq;
use embassy_rp::pio::{Common, Config, FifoJoin, Instance, LoadedProgram, PioPin, ShiftConfig, ShiftDirection, StateMachine};
use embassy_rp::pio::program::{Assembler, JmpCondition, OutDestination, SetDestination, SideSet};
use embassy_time::Timer;
use fixed::types::U24F8;
use smart_leds::RGBA;

const T1: u8 = 2; // start bit
const T2: u8 = 5; // data bit
const T3: u8 = 3; // stop bit
const CYCLES_PER_BIT: u32 = (T1 + T2 + T3) as u32;

pub(super) struct CustomPioWsProgram<'a, P: Instance> {
    prg: LoadedProgram<'a, P>,
}
impl<'a, P: Instance> CustomPioWsProgram<'a, P> {
    pub(super) fn new(common: &mut Common<'a, P>) -> Self {
        let side_set = SideSet::new(false, 1, false);
        let mut a: Assembler<32> = Assembler::new_with_side_set(side_set);

        let mut wrap_target = a.label();
        let mut wrap_source = a.label();
        let mut do_zero = a.label();
        a.set_with_side_set(SetDestination::PINDIRS, 1, 0);
        a.bind(&mut wrap_target);

        // Do stop bit
        a.out_with_delay_and_side_set(OutDestination::X, 1, T3 - 1, 0);

        // Do start bit
        a.jmp_with_delay_and_side_set(JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);

        // Do data bit = 1
        a.jmp_with_delay_and_side_set(JmpCondition::Always, &mut wrap_target, T2 - 1, 1);
        a.bind(&mut do_zero);

        // Do data bit = 0
        a.nop_with_delay_and_side_set(T2 - 1, 0);
        a.bind(&mut wrap_source);

        let prg = a.assemble_with_wrap(wrap_source, wrap_target);
        let prg = common.load_program(&prg);

        Self { prg }
    }
}

pub(super) struct CustomPioWs<'d, P: Instance, const S: usize, const N: usize> {
    dma: PeripheralRef<'d, AnyChannel>,
    sm: StateMachine<'d, P, S>,
}
impl<'d, P: Instance, const S: usize, const N: usize> CustomPioWs<'d, P, S, N> {
    pub(super) fn new(
        pio: &mut Common<'d, P>,
        mut sm: StateMachine<'d, P, S>,
        dma: impl Peripheral<P = impl Channel> + 'd,
        pin: impl PioPin,
        program: &CustomPioWsProgram<'d, P>,
    ) -> Self {
        into_ref!(dma);

        let mut cfg = Config::default();

        let out_pin = pio.make_pio_pin(pin);
        cfg.set_out_pins(&[&out_pin]);
        cfg.set_set_pins(&[&out_pin]);
        cfg.use_program(&program.prg, &[&out_pin]);


        // Clock config, measured in kHz to avoid overflows
        let clock_freq = U24F8::from_num(clk_sys_freq() / 1000);
        let ws2812_freq = U24F8::from_num(800);
        let bit_freq = ws2812_freq * CYCLES_PER_BIT;
        cfg.clock_divider = clock_freq / bit_freq;


        // FIFO config
        cfg.fifo_join = FifoJoin::TxOnly;
        cfg.shift_out = ShiftConfig {
            auto_fill: true,
            threshold: 32,
            direction: ShiftDirection::Left,
        };

        sm.set_config(&cfg);
        sm.set_enable(true);

        Self {
            dma: dma.map_into(),
            sm,
        }
    }

    pub(super) async fn write(&mut self, colors: &[RGBA<u8>; N]) {
        let mut words = [0u32; N];
        for (i, value) in colors.iter().enumerate() {
            let r = value.r as u32;
            let b = value.b as u32;
            let g = value.g as u32;
            let a = value.a as u32;
            words[i] = (g << 24) | (r << 16) | (b << 8) | a;
        }

        self.sm.tx().dma_push(self.dma.reborrow(), &words, false).await;
        Timer::after_micros(55).await;
    }
}
