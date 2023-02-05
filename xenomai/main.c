#include <stdio.h>
#include <native/task.h>
#include <native/timer.h>
#include <sys/mman.h>
#include <stdlib.h>
#include <unistd.h>
#include "gpio.h"
//#include <stdint-gcc.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>



#define TASK_STKSZ	0	//default stack size
#define TASK_PRIO	99	//0-99
#define TASK_MODE	0	//no flags

static volatile uint32_t *gpio_base = NULL;

void motor_control(void *arg);

int main(int argc, char** argv) {
	
  

	
	int i;
	int err;
       int time_s = 10;
   
  int fd = open("/dev/mem", O_RDWR | O_SYNC);
  if(fd < 0){
    printf("Failed to open /dev/mem \n");
    exit(1);
  }

  const uint32_t rpi_gpio_base = 0x20000000 + 0x200000;
  const uint32_t rpi_block_size = 4096;
  void *gpio_mmap = mmap(NULL,rpi_block_size,
      PROT_READ | PROT_WRITE, MAP_SHARED,fd,rpi_gpio_base);

  if(gpio_mmap == MAP_FAILED){
    printf("GPIO mmap  failed \n");
    exit(1);
  }
  close(fd);
  gpio_base = (volatile uint32_t *)gpio_mmap;

  /* step2: GPIO*/
  const int led = 17;

  //GPIO Function Set 
  const uint32_t index = 0 + led/10;// 0: index of GPFSEL0
  const unsigned char shift = (led%10)*3;
  const uint32_t mask = ~(0x7 << shift);
  gpio_base[index] = (gpio_base[index] & mask) | (1 << shift);//1: OUTPUT
	
	mlockall(MCL_CURRENT | MCL_FUTURE);
	RT_TASK rt_task;

	err = rt_task_spawn( &rt_task, "rt_motor_control", TASK_STKSZ, TASK_PRIO, TASK_MODE, &motor_control, NULL);
	
	if(err) {
		rt_task_delete(&rt_task);
		exit(1);
	}
    
	getchar();
	return 0;
}

void motor_control(void *arg) {
 const int led = 17;
int time_s = 1900;
  const uint32_t rpi_gpio_base = 0x20000000 + 0x200000;
  const uint32_t rpi_block_size = 4096;
      /* step3: GPIO */
  while(1) {
    gpio_base[7] = 1<<led;//7: index of GPSET0
    usleep(time_s);
    gpio_base[10] = 1<<led;//10: index of GPCLR0
    usleep(time_s);
  }
    /* step4: ）*/
  munmap((void *)gpio_base,rpi_block_size);
  exit(0);
}
