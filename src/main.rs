
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::hal::digital::v2::OutputPin;
use stm32h7xx_hal::{pac, prelude::*};
use panic_rtt_core::{self, rprintln, rtt_init_print};

use stm32h7xx_hal::pwr::VoltageScale;
use stm32h7xx_hal::time::MegaHertz;
use stm32h7xx_hal::rcc::PllConfigStrategy;

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockTrim);
    rprintln!("--> MAIN --");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // #[cfg(not(feature = "breakout"))]
    let hse_xtal_freq: MegaHertz = 12.mhz();
    // #[cfg(feature = "breakout")]
    //     let hse_xtal_freq: MegaHertz = 25.mhz();

    // --- Clock configuration
    // Set up the system clock
    // For OpenMV H7 we know we have:
    // - 12 MHz xtal HSE
    // - SYSCLK of 480 MHz (processor max)
    // - HCLK of SYSCLK/2 (240 MHz)
    // - (PCLK1, PCLK2, PCLK3, PCLK4) is HCLK/2 (120 MHz)
    // - PLL1P = PLL1_VCO/2  = 960 MHz / 2   = 480 MHz
    // - PLL1Q = PLL1_VCO/4  = 960 MHz / 4   = 240 MHz
    // - PLL1R = PLL1_VCO/8  = 960 MHz / 8   = 120 MHz
    const LE_SYSCLK: u32 = 480;
    const LE_HCLK: u32 = LE_SYSCLK / 2;
    const LE_PCLK: u32 = LE_HCLK / 2;
    let rcc = dp
        .RCC
        .constrain()
        .use_hse(hse_xtal_freq) // OpenMV H7 has 12 MHz xtal HSE
        .sysclk(LE_SYSCLK.mhz())
        .hclk(LE_HCLK.mhz())
        .pll1_p_ck(480.mhz())
        .pll1_q_ck(240.mhz())
        .pll1_r_ck(120.mhz())
        .pll1_strategy(PllConfigStrategy::Iterative)
        .pclk1(LE_PCLK.mhz())
        .pclk2(LE_PCLK.mhz())
        .pclk3(LE_PCLK.mhz())
        .pclk4(LE_PCLK.mhz());

    let pwr = dp.PWR.constrain();
    let _vos = pwr.freeze();

    //vos defaults to Scale1 but needs to upgrade to Scale0 to boost to 480 MHz
    let vos = VoltageScale::Scale0; //may force higher? or just allow asserts to pass?
    let mut ccdr = rcc.freeze(vos, &dp.SYSCFG);

    let gpioc = dp.GPIOC.split(&mut ccdr.ahb4);
    let mut led = gpioc.pc0.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500_u16);

        led.set_low().unwrap();
        delay.delay_ms(500_u16);
        rprintln!("/");
    }
}
