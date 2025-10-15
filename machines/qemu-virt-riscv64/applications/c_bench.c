// #include <stdio.h>
// #include <unistd.h>
// #include <sys/time.h>
// #include <rtthread.h>
// #include <rtdbg.h>

// int c_bench(void) {
//     struct timeval tv_begin, tv_end;
//     gettimeofday(&tv_begin,NULL);
//     for (int i = 0; i < 1; i++) {
// 	    rt_kprintf("C program: Hello world! %d\n", i);
//     }
//     gettimeofday(&tv_end,NULL);
//     double milisecs = (tv_end.tv_sec - tv_begin.tv_sec) * 1000.0 + (tv_end.tv_usec - tv_begin.tv_usec) / 1000.0;
//     rt_kprintf("Time: %fms\n", milisecs);
//     return 0;
// }

// INIT_ENV_EXPORT(c_bench);
