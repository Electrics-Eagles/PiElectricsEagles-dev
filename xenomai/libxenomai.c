#include <stdio.h>
#include <native/task.h>
#include <native/timer.h>
#include <sys/mman.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>

#define TASK_STKSZ 0  // Default stack size
#define TASK_PRIO 99  // 0-99
#define TASK_MODE 0   // No flags

static volatile uint32_t *gpio_base = NULL;

void rust_task_real_time(void *arg);

int main(int argc, char **argv) {
    int err;
    int fd = open("/dev/mem", O_RDWR | O_SYNC);  // Open /dev/mem with read/write access and synchronous I/O
    if (fd < 0) {
        printf("Failed to open /dev/mem\n");
        exit(1);
    }
    mlockall(MCL_CURRENT | MCL_FUTURE);  // Lock current and future memory pages into RAM to prevent swapping
    RT_TASK rt_task;
    // Create a real-time task with the specified priority, stack size, and function to execute
    err = rt_task_spawn(&rt_task, "pidronecode", TASK_STKSZ, TASK_PRIO, TASK_MODE, &rust_task_real_time, NULL);
    if (err) {
        rt_task_delete(&rt_task);  // Delete the real-time task
        exit(1);
    }
    getchar();  // Wait for a keypress before exiting
    return 0;
}

void rust_task_real_time(void *arg) {
    exit(0);  // Exit the real-time task
}
