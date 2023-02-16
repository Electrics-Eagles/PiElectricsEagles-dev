#include <stdio.h>
#include <native/task.h>
#include <native/timer.h>
#include <sys/mman.h>
#include <stdlib.h>
#include <unistd.h>
//#include "gpio.h"
//#include <stdint-gcc.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/mman.h>


#define TASK_STKSZ	0	//default stack size
#define TASK_PRIO	99	//0-99
#define TASK_MODE	0	//no flags

static volatile uint32_t *gpio_base = NULL;

void rust_task_real_time(void *arg);

int main(int argc, char** argv) {
	
	int i;
	int err;
    int time_s = 10;
   
  int fd = open("/dev/mem", O_RDWR | O_SYNC);
  if(fd < 0){
    printf("Failed to open /dev/mem \n");
    exit(1);
  }
	mlockall(MCL_CURRENT | MCL_FUTURE);
	RT_TASK rt_task;
	err = rt_task_spawn( &rt_task, "pidronecode", TASK_STKSZ, TASK_PRIO, TASK_MODE, &rust_task_real_time, NULL);
	if(err) {
		rt_task_delete(&rt_task);
		exit(1);
	}
	getchar();
	return 0;
}

void rust_task_real_time(void *arg) {

  exit(0);
}