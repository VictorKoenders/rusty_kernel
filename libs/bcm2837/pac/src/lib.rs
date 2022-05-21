#![doc = "Peripheral access API for BCM2837_LPA microcontrollers (generated using svd2rust v0.24.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Mailboxes for talking to/from VideoCore"]
pub struct VCMAILBOX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VCMAILBOX {}
impl VCMAILBOX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vcmailbox::RegisterBlock = 0x3f00_b880 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vcmailbox::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VCMAILBOX {
    type Target = vcmailbox::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VCMAILBOX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCMAILBOX").finish()
    }
}
#[doc = "Mailboxes for talking to/from VideoCore"]
pub mod vcmailbox;
#[doc = "Broadcom Clock Manager"]
pub struct CM_PCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CM_PCM {}
impl CM_PCM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cm_pcm::RegisterBlock = 0x3f10_1098 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cm_pcm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CM_PCM {
    type Target = cm_pcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CM_PCM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM_PCM").finish()
    }
}
#[doc = "Broadcom Clock Manager"]
pub mod cm_pcm;
#[doc = "Broadcom Clock Manager"]
pub struct CM_PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CM_PWM {}
impl CM_PWM {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cm_pcm::RegisterBlock = 0x3f10_10a0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cm_pcm::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CM_PWM {
    type Target = cm_pcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CM_PWM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM_PWM").finish()
    }
}
#[doc = "Broadcom Clock Manager"]
pub use cm_pcm as cm_pwm;
#[doc = "Pin level and mux control"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x3f20_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "Pin level and mux control"]
pub mod gpio;
#[doc = "Broadcom System Timer"]
pub struct SYSTMR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTMR {}
impl SYSTMR {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const systmr::RegisterBlock = 0x3f00_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const systmr::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTMR {
    type Target = systmr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTMR").finish()
    }
}
#[doc = "Broadcom System Timer"]
pub mod systmr;
#[doc = "ARM Prime Cell PL011"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x3f20_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "ARM Prime Cell PL011"]
pub mod uart0;
#[doc = "Broadcom SPI Controller"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x3f20_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "Broadcom SPI Controller"]
pub mod spi0;
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub struct BSC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSC0 {}
impl BSC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bsc0::RegisterBlock = 0x3f20_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bsc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BSC0 {
    type Target = bsc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BSC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSC0").finish()
    }
}
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub mod bsc0;
#[doc = "Broadcom PWM"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pwm0::RegisterBlock = 0x3f20_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PWM0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0").finish()
    }
}
#[doc = "Broadcom PWM"]
pub mod pwm0;
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub struct BSC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSC1 {}
impl BSC1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bsc0::RegisterBlock = 0x3f80_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bsc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BSC1 {
    type Target = bsc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BSC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSC1").finish()
    }
}
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub use bsc0 as bsc1;
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub struct BSC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSC2 {}
impl BSC2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bsc0::RegisterBlock = 0x3f80_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bsc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BSC2 {
    type Target = bsc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BSC2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSC2").finish()
    }
}
#[doc = "Broadcom Serial Controller (I2C compatible)"]
pub use bsc0 as bsc2;
#[doc = "Three auxiliary peripherals"]
pub struct AUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX {}
impl AUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aux::RegisterBlock = 0x3f21_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AUX {
    type Target = aux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX").finish()
    }
}
#[doc = "Three auxiliary peripherals"]
pub mod aux;
#[doc = "Mini UART"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart1::RegisterBlock = 0x3f21_5040 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "Mini UART"]
pub mod uart1;
#[doc = "Aux SPI"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x3f21_5080 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "Aux SPI"]
pub mod spi1;
#[doc = "Aux SPI"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x3f21_50c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
#[doc = "Aux SPI"]
pub use spi1 as spi2;
#[doc = "Broadcom Legacy Interrupt Controller"]
pub struct LIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LIC {}
impl LIC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lic::RegisterBlock = 0x3f00_b000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lic::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LIC {
    type Target = lic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LIC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LIC").finish()
    }
}
#[doc = "Broadcom Legacy Interrupt Controller"]
pub mod lic;
#[doc = "USB on the go high speed"]
pub struct USB_OTG_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG_GLOBAL {}
impl USB_OTG_GLOBAL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb_otg_global::RegisterBlock = 0x3f98_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_otg_global::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB_OTG_GLOBAL {
    type Target = usb_otg_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB_OTG_GLOBAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTG_GLOBAL").finish()
    }
}
#[doc = "USB on the go high speed"]
pub mod usb_otg_global;
#[doc = "USB on the go high speed"]
pub struct USB_OTG_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG_HOST {}
impl USB_OTG_HOST {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb_otg_host::RegisterBlock = 0x3f98_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_otg_host::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB_OTG_HOST {
    type Target = usb_otg_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB_OTG_HOST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTG_HOST").finish()
    }
}
#[doc = "USB on the go high speed"]
pub mod usb_otg_host;
#[doc = "USB on the go high speed"]
pub struct USB_OTG_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG_DEVICE {}
impl USB_OTG_DEVICE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb_otg_device::RegisterBlock = 0x3f98_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_otg_device::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB_OTG_DEVICE {
    type Target = usb_otg_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB_OTG_DEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTG_DEVICE").finish()
    }
}
#[doc = "USB on the go high speed"]
pub mod usb_otg_device;
#[doc = "USB on the go high speed power control"]
pub struct USB_OTG_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB_OTG_PWRCLK {}
impl USB_OTG_PWRCLK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usb_otg_pwrclk::RegisterBlock = 0x3f98_0e00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb_otg_pwrclk::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USB_OTG_PWRCLK {
    type Target = usb_otg_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USB_OTG_PWRCLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTG_PWRCLK").finish()
    }
}
#[doc = "USB on the go high speed power control"]
pub mod usb_otg_pwrclk;
#[doc = "Arasan SD3.0 Host AHB eMMC 4.4"]
pub struct EMMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMMC {}
impl EMMC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const emmc::RegisterBlock = 0x3f30_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emmc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EMMC {
    type Target = emmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EMMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMMC").finish()
    }
}
#[doc = "Arasan SD3.0 Host AHB eMMC 4.4"]
pub mod emmc;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "VCMAILBOX"]
    pub VCMAILBOX: VCMAILBOX,
    #[doc = "CM_PCM"]
    pub CM_PCM: CM_PCM,
    #[doc = "CM_PWM"]
    pub CM_PWM: CM_PWM,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "SYSTMR"]
    pub SYSTMR: SYSTMR,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "BSC0"]
    pub BSC0: BSC0,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "BSC1"]
    pub BSC1: BSC1,
    #[doc = "BSC2"]
    pub BSC2: BSC2,
    #[doc = "AUX"]
    pub AUX: AUX,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "LIC"]
    pub LIC: LIC,
    #[doc = "USB_OTG_GLOBAL"]
    pub USB_OTG_GLOBAL: USB_OTG_GLOBAL,
    #[doc = "USB_OTG_HOST"]
    pub USB_OTG_HOST: USB_OTG_HOST,
    #[doc = "USB_OTG_DEVICE"]
    pub USB_OTG_DEVICE: USB_OTG_DEVICE,
    #[doc = "USB_OTG_PWRCLK"]
    pub USB_OTG_PWRCLK: USB_OTG_PWRCLK,
    #[doc = "EMMC"]
    pub EMMC: EMMC,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            VCMAILBOX: VCMAILBOX {
                _marker: PhantomData,
            },
            CM_PCM: CM_PCM {
                _marker: PhantomData,
            },
            CM_PWM: CM_PWM {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            SYSTMR: SYSTMR {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            BSC0: BSC0 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            BSC1: BSC1 {
                _marker: PhantomData,
            },
            BSC2: BSC2 {
                _marker: PhantomData,
            },
            AUX: AUX {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            LIC: LIC {
                _marker: PhantomData,
            },
            USB_OTG_GLOBAL: USB_OTG_GLOBAL {
                _marker: PhantomData,
            },
            USB_OTG_HOST: USB_OTG_HOST {
                _marker: PhantomData,
            },
            USB_OTG_DEVICE: USB_OTG_DEVICE {
                _marker: PhantomData,
            },
            USB_OTG_PWRCLK: USB_OTG_PWRCLK {
                _marker: PhantomData,
            },
            EMMC: EMMC {
                _marker: PhantomData,
            },
        }
    }
}
