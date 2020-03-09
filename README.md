# low level rng
the random analog generator (RNG) clock (≤48 MHz) is coming from PLL48CLK.

set rcc_cr_pllon to 1?
set hsi on

Since the main-PLL configuration parameters cannot be changed once PLL is enabled, it is
recommended to configure PLL before enabling it (selection of the HSI or HSE oscillator as
PLL clock source, and configuration of division factors M, N, P, and Q).

## RCC_CFGR
set this
https://sites.google.com/site/johnkneenmicrocontrollers/clocks/clk407

## default values
PLLQ = 4
PLLN = 0b0010_0100_00 = 144
PLLM = 0x10 = 16

PLLN/PLLM = 144/16 = 9
