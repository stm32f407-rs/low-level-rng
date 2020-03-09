# low level rng
get random values from rng by writing to memory-mapped registers.

HSI is used as PLL source.

Since the main-PLL configuration parameters cannot be changed once PLL is enabled, it is
recommended to configure PLL before enabling it (selection of the HSI or HSE oscillator as
PLL clock source, and configuration of division factors M, N, P, and Q).

## default values
These PLL default values are compatible with rng so there is no need to change them.

PLLQ = 4
PLLN = 144
PLLM = 16

PLLN/PLLM = 144/16 = 9
