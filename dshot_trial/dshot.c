#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>

#define BCM2708_PERI_BASE        0x3F000000
#define GPIO_BASE                (BCM2708_PERI_BASE + 0x200000) /* GPIO controller */
#define PAGE_SIZE (4*1024)
#define BLOCK_SIZE (4*1024)

// GPIO setup macros. Always use INP_GPIO(x) before using OUT_GPIO(x) or SET_GPIO_ALT(x,y)
#define INP_GPIO(g) *(gpio+((g)/10)) &= ~(7<<(((g)%10)*3))
#define OUT_GPIO(g) *(gpio+((g)/10)) |=  (1<<(((g)%10)*3))
#define SET_GPIO_ALT(g,a) *(gpio+(((g)/10))) |= (((a)<=3?(a)+4:(a)==4?3:2)<<(((g)%10)*3))

#define GPIO_SET *(gpio+7)  // sets   bits which are 1 ignores bits which are 0
#define GPIO_CLR *(gpio+10) // clears bits which are 1 ignores bits which are 0

#define GET_GPIO(g) (*(gpio+13)&(1<<g)) // 0 if LOW, (1<<g) if HIGH

#define GPIO_PULL *(gpio+37) // Pull up/pull down
#define GPIO_PULLCLK0 *(gpio+38) // Pull up/pull down clock

#define NUM_PINS 27

void *gpio_map;
volatile unsigned *gpio;

// dshot150 timings from experiments
const int T0H = 2848;
const int T0L = 4870;
const int T1H = 5857;
const int T1L = 1861;

int pins[NUM_PINS] = {0};
int pin_num = 0;
int first_time = 1;

//
// Set up a memory region to access GPIO
//
void setup_io()
{
   int mem_fd;

   /* open /dev/mem */
   if ((mem_fd = open("/dev/mem", O_RDWR|O_SYNC) ) < 0) {
      printf("can't open /dev/mem \n");
      exit(-1);
   }

   /* mmap GPIO */
   gpio_map = mmap(
      NULL,             // Any address in our space will do
      BLOCK_SIZE,       // Map length
      PROT_READ|PROT_WRITE, // Enable reading & writing to mapped memory
      MAP_SHARED,       // Shared with other processes
      mem_fd,           // File to map
      GPIO_BASE         // Offset to GPIO peripheral
   );

   close(mem_fd); // No need to keep mem_fd open after mmap

   if (gpio_map == MAP_FAILED) {
      printf("mmap error %p\n", gpio_map);
      exit(-1);
   }

   // Always use volatile pointer!
   gpio = (volatile unsigned *)gpio_map;
}

inline void wait_cycles(int l) {
    asm volatile("0:" "SUBS %[count], #1;" "BNE 0b;" : [count] "+r"(l));
}

inline void low(int pin) {
    GPIO_SET = 1 << pin;
    wait_cycles(T0H);
    GPIO_CLR = 1 << pin;
    wait_cycles(T0L);
}

inline void high(int pin) {
    GPIO_SET = 1 << pin;
    wait_cycles(T1H);
    GPIO_CLR = 1 << pin;
    wait_cycles(T1L);
}

inline int add_checksum_and_telemetry(int packet, int telem) {
    int packet_telemetry = (packet << 1) | (telem & 1);
    int i;
    int csum = 0;
    int csum_data = packet_telemetry;
    for (i = 0; i < 3; i++) {
        csum ^=  csum_data;   // xor data by nibbles
        csum_data >>= 4;
    }
    csum &= 0xf;
    // Append checksum
    int packet_telemetry_checksum = (packet_telemetry << 4) | csum;

    return packet_telemetry_checksum;
}

inline void send_throttle(int pin, int throttle, int telem) {
    int throttle_packet = add_checksum_and_telemetry(throttle, telem);
    int array_index = 0;
    int i;
    for (i = 15; i >= 0; i--) {
        if ((throttle_packet >> i) & 1) {
            high(pin);
        } else {
            low(pin);
        }
    }
}

int main() {
    int value;
    int pin;

    // Initialize GPIO
    setup_io();

    // Example usage: send throttle value 1000 on pin 17
    value = 1000;
    pin = 17;

    if (!pins[pin]) {
        pins[pin] = 1;
        INP_GPIO(pin); // Must use INP_GPIO before using OUT_GPIO
        OUT_GPIO(pin);
        GPIO_CLR = 1 << pin;
    }

    send_throttle(pin, value, 0);

    return 0;
}
