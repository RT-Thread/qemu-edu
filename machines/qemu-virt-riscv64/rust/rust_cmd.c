/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-20     RT-Thread    First version
 *
 * Description: Rust component MSH command registration
 *              Provides access interface to Rust modular APIs
 */

#include <rtthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/time.h>
#include <rtdbg.h>

/* ============== Rust function declarations ============== */

/* Initialization */
extern int rust_init(void);

/* hello module */
#ifdef RUST_EXAMPLE_HELLO
extern void rust_hello(void);
extern void rust_hello_with_name(const char *name);
extern void rust_hello_rust_style(void);
#endif

/* printf_demo module */
#ifdef RUST_EXAMPLE_PRINTF
extern void rust_printf_demo(void);
extern int rust_sprintf_demo(void);
extern void rust_vec_demo(void);
#endif

/* memory_demo module */
#ifdef RUST_EXAMPLE_MEMORY
extern int rust_add(int a, int b);
extern int rust_multiply(int a, int b);
extern int rust_memcpy_test(void *dest, const void *src, size_t size);
extern void rust_memset_demo(void);
extern void rust_memcmp_demo(void);
extern void rust_malloc_demo(void);
extern void rust_rt_malloc_demo(void);
extern void rust_memory_demo_all(void);
#endif

/* thread_demo module */
#ifdef RUST_EXAMPLE_THREAD
extern void rust_thread_create_demo(void);
extern void rust_thread_self_demo(void);
extern void rust_thread_sleep_demo(void);
extern void rust_thread_wrapper_demo(void);
extern void rust_thread_concurrent_demo(void);
extern void rust_thread_demo_all(void);
#endif

/* mutex_demo module */
#ifdef RUST_EXAMPLE_MUTEX
extern void rust_mutex_basic_demo(void);
extern void rust_mutex_named_demo(void);
extern void rust_mutex_trylock_demo(void);
extern void rust_mutex_atomic_demo(void);
extern void rust_mutex_concurrent_demo(void);
extern void rust_mutex_types_demo(void);
extern void rust_mutex_demo_all(void);
#endif

/* sem_demo module */
#ifdef RUST_EXAMPLE_SEM
extern void rust_semaphore_basic_demo(void);
extern void rust_semaphore_producer_consumer_demo(void);
extern void rust_semaphore_multi_thread_demo(void);
extern void rust_semaphore_demo_all(void);
#endif

/* mq_demo module */
#ifdef RUST_EXAMPLE_MQ
extern void rust_mq_demo(void);
#endif

/* vec_demo module */
#ifdef RUST_EXAMPLE_VEC
extern void rust_vec_demo(void);
extern void rust_vec_demo_all(void);
#endif

/* dl_demo module */
#ifdef RUST_EXAMPLE_DL
extern void rust_dl_open_demo(void);
extern void rust_dl_sym_demo(void);
extern void rust_dl_call_demo(void);
extern void rust_dl_error_demo(void);
extern void rust_dl_demo_all(void);
#endif
/* ==============  bench test implementation ============== */
#ifdef RUST_BENCH_TEST
extern void rust_bench_test(void);
int c_bench(void) {
    struct timeval tv_begin, tv_end;
    gettimeofday(&tv_begin,NULL);
    for (int i = 0; i < 1; i++) {
	    rt_kprintf("C program: Hello world! %d\n", i);
    }
    gettimeofday(&tv_end,NULL);
    double milisecs = (tv_end.tv_sec - tv_begin.tv_sec) * 1000.0 + (tv_end.tv_usec - tv_begin.tv_usec) / 1000.0;
    rt_kprintf("Time: %fms\n", milisecs);
    return 0;
}
#endif

/* ============== MSH command implementation ============== */

/* Basic command: hello */
#ifdef RUST_EXAMPLE_HELLO
static int cmd_rust_hello(int argc, char **argv)
{
    if (argc == 1)
    {
        rust_hello();
    }
    else if (argc == 2)
    {
        rust_hello_with_name(argv[1]);
    }
    else
    {
        printf("Usage: rust_hello [name]\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_hello, rust_hello, Rust hello command);
#endif

/* Arithmetic command: add */
#ifdef RUST_EXAMPLE_MEMORY
static int cmd_rust_add(int argc, char **argv)
{
    if (argc < 3)
    {
        printf("Usage: rust_add <num1> <num2>\n");
        printf("Example: rust_add 100 200\n");
        return -1;
    }
    
    int a = atoi(argv[1]);
    int b = atoi(argv[2]);
    int result = rust_add(a, b);
    printf("%d + %d = %d\n", a, b, result);
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_add, rust_add, Add two numbers using Rust);

/* Arithmetic command: multiply */
static int cmd_rust_mul(int argc, char **argv)
{
    if (argc < 3)
    {
        printf("Usage: rust_mul <num1> <num2>\n");
        printf("Example: rust_mul 10 20\n");
        return -1;
    }
    
    int a = atoi(argv[1]);
    int b = atoi(argv[2]);
    int result = rust_multiply(a, b);
    printf("%d * %d = %d\n", a, b, result);
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_mul, rust_mul, Multiply two numbers using Rust);
#endif /* RUST_EXAMPLE_MEMORY */

/* Memory demonstration */
#ifdef RUST_EXAMPLE_MEMORY
static int cmd_rust_memory(int argc, char **argv)
{
    rust_memory_demo_all();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_memory, rust_memory, Demonstrate memory operations);
#endif

/* Thread demonstration */
#ifdef RUST_EXAMPLE_THREAD
static int cmd_rust_thread(int argc, char **argv)
{
    if (argc == 1)
    {
        rust_thread_demo_all();
    }
    else if (strcmp(argv[1], "create") == 0)
    {
        rust_thread_create_demo();
    }
    else if (strcmp(argv[1], "self") == 0)
    {
        rust_thread_self_demo();
    }
    else if (strcmp(argv[1], "sleep") == 0)
    {
        rust_thread_sleep_demo();
    }
    else if (strcmp(argv[1], "wrapper") == 0)
    {
        rust_thread_wrapper_demo();
    }
    else if (strcmp(argv[1], "concurrent") == 0)
    {
        rust_thread_concurrent_demo();
    }
    else
    {
        printf("Usage: rust_thread [create|self|sleep|wrapper|concurrent]\n");
        printf("  Without arguments: run all demos\n");
        printf("  create  - demonstrate thread creation\n");
        printf("  self    - show current thread info\n");
        printf("  sleep   - demonstrate thread sleep\n");
        printf("  wrapper - demonstrate Rust thread wrapper\n");
        printf("  concurrent - demonstrate multiple threads concurrent execution\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_thread, rust_thread, RT-Thread operations demo);
#endif

/* Mutex demonstration */
#ifdef RUST_EXAMPLE_MUTEX
static int cmd_rust_mutex(int argc, char **argv)
{
    if (argc == 1)
    {
        rust_mutex_demo_all();
    }
    else if (strcmp(argv[1], "basic") == 0)
    {
        rust_mutex_basic_demo();
    }
    else if (strcmp(argv[1], "named") == 0)
    {
        rust_mutex_named_demo();
    }
    else if (strcmp(argv[1], "trylock") == 0)
    {
        rust_mutex_trylock_demo();
    }
    else if (strcmp(argv[1], "atomic") == 0)
    {
        rust_mutex_atomic_demo();
    }
    else if (strcmp(argv[1], "concurrent") == 0)
    {
        rust_mutex_concurrent_demo();
    }
    else if (strcmp(argv[1], "types") == 0)
    {
        rust_mutex_types_demo();
    }
    else
    {
        printf("Usage: rust_mutex [basic|named|trylock|atomic|concurrent|types]\n");
        printf("  basic      - Basic mutex creation and lock/unlock\n");
        printf("  named      - Named mutex example\n");
        printf("  trylock    - Try lock with timeout example\n");
        printf("  atomic     - Atomic mutex for interrupt context\n");
        printf("  concurrent - Multi-threaded mutex contention\n");
        printf("  types      - Sleep vs Atomic mutex comparison\n");
        printf("  (no args)  - Run all mutex demos\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_mutex, rust_mutex, RT-Thread mutex operations demo);
#endif

/* sem_demo module */
#ifdef RUST_EXAMPLE_SEM
static int cmd_rust_sem(int argc, char **argv)
{
    if (argc == 1)
    {
        rust_semaphore_demo_all();
    }
    else if (strcmp(argv[1], "basic") == 0)
    {
        rust_semaphore_basic_demo();
    }
    else if (strcmp(argv[1], "producer_consumer") == 0)
    {
        rust_semaphore_producer_consumer_demo();
    }
    else if (strcmp(argv[1], "multi_thread") == 0)
    {
        rust_semaphore_multi_thread_demo();
    }
    else
    {
        printf("Usage: rust_sem [basic|producer_consumer|multi_thread]\n");
        printf("  basic - Basic semaphore creation and wait/signal\n");
        printf("  producer_consumer - Producer-Consumer semaphore demo\n");
        printf("  multi_thread - Multiple threads synchronization demo\n");
        printf("  (no args) - Run all semaphore demos\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_sem, rust_sem, RT-Thread semaphore operations demo);
#endif
/* message queue demonstration */
#ifdef RUST_EXAMPLE_MQ
static int cmd_rust_mq(int argc, char **argv)
{
    rust_mq_demo();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_mq, rust_mq, RT-Thread message queue operations demo);
#endif
/* dl_demo module */
#ifdef RUST_EXAMPLE_DL
static int cmd_rust_dl(int argc, char **argv)
{
    if (argc == 1)
    {
        rust_dl_demo_all();
    }
    else if (strcmp(argv[1], "open") == 0)
    {
        rust_dl_open_demo();
    }
    else if (strcmp(argv[1], "sym") == 0)
    {
        rust_dl_sym_demo();
    }
    else if (strcmp(argv[1], "call") == 0)
    {
        rust_dl_call_demo();
    }
    else if (strcmp(argv[1], "error") == 0)
    {
        rust_dl_error_demo();
    }
    else
    {
        printf("Usage: rust_dl [open|sym|call|error]\n");
        printf("  Without arguments: run all demos\n");
        printf("  open  - demonstrate dlopen/dlclose\n");
        printf("  sym   - demonstrate dlsym symbol resolution\n");
        printf("  call  - demonstrate function calls through dlsym\n");
        printf("  error - demonstrate error handling\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_dl, rust_dl, Demonstrate libdl operations);
#endif
/* Printf demonstration */
#ifdef RUST_EXAMPLE_PRINTF
static int cmd_rust_printf(int argc, char **argv)
{
    rust_printf_demo();
    rust_sprintf_demo();
    rust_vec_demo();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_printf, rust_printf, Demonstrate printf operations);
#endif

/* bench test command */
#ifdef RUST_BENCH_TEST
static int cmd_bench_test(int argc, char **argv)
{
    printf("\n=== C & Rust Bench Test ===\n");
    rust_bench_test();
    c_bench();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_bench_test, bench_test, Run Rust bench test);
#endif

/* test command */
static int cmd_rust_test(int argc, char **argv)
{
    printf("\n=== Rust Component Test Suite ===\n");
    
    /* 1. Hello test */
    printf("\n1. Hello Test:\n");
    #ifdef RUST_EXAMPLE_HELLO
    rust_hello();
    rust_hello_rust_style();
    #else
    printf("   (hello example disabled)\n");
    #endif
    
    /* 2. Printf test */
    printf("\n2. Printf Test:\n");
    #ifdef RUST_EXAMPLE_PRINTF
    rust_printf_demo();
    #else
    printf("   (printf example disabled)\n");
    #endif
    
    /* 3. Arithmetic test */
    printf("\n3. Arithmetic Test:\n");
    #ifdef RUST_EXAMPLE_MEMORY
    printf("   42 + 58 = %d\n", rust_add(42, 58));
    printf("   10 * 20 = %d\n", rust_multiply(10, 20));
    #else
    printf("   (memory/arithmetic example disabled)\n");
    #endif
    
    /* 4. Memory test */
    printf("\n4. Memory Test:\n");
    #ifdef RUST_EXAMPLE_MEMORY
    char src[] = "Hello";
    char dest[10];
    if (rust_memcpy_test(dest, src, strlen(src) + 1))
    {
        printf("   memcpy passed: '%s'\n", dest);
    }
    #else
    printf("   (memory example disabled)\n");
    #endif

    /* 5. Thread test */
    printf("\n5. Thread Test:\n");
    #ifdef RUST_EXAMPLE_THREAD
    rust_thread_demo_all();
    #else
    printf("   (thread example disabled)\n");
    #endif

    /* 6. Mutex test */
    printf("\n6. Mutex Test:\n");
    #ifdef RUST_EXAMPLE_MUTEX
    rust_mutex_demo_all();
    #else
    printf("   (mutex example disabled)\n");
    #endif

    /* 7. Semaphore test */
    printf("\n7. Semaphore Test:\n");
    #ifdef RUST_EXAMPLE_SEM
    rust_semaphore_demo_all();
    #else
    printf("   (semaphore example disabled)\n");
    #endif

    /* 8. Message queue test */
    printf("\n8. Message Queue Test:\n");
    #ifdef RUST_EXAMPLE_MQ
    rust_mq_demo();
    #else
    printf("   (message queue example disabled)\n");
    #endif
    
    /* 9. Dynamic library test */
    printf("\n9. Dynamic Library Test:\n");
    #ifdef RUST_EXAMPLE_DL
    rust_dl_demo_all();
    #else
    printf("   (dynamic library example disabled)\n");
    #endif
    
    printf("\n=== All tests completed ===\n");
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_test, rust_test, Run Rust component test suite);

/* Help command */
static int cmd_rust_help(int argc, char **argv)
{
    printf("\nRust Component Commands:\n");
    #ifdef RUST_EXAMPLE_HELLO
    printf("  rust_hello [name]    - Say hello\n");
    #endif
    #ifdef RUST_EXAMPLE_MEMORY
    printf("  rust_add <n1> <n2>   - Add two numbers\n");
    printf("  rust_mul <n1> <n2>   - Multiply two numbers\n");
    printf("  rust_memory          - Memory operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_PRINTF
    printf("  rust_printf          - Printf operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_THREAD
    printf("  rust_thread [opt]    - Thread operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_MUTEX
    printf("  rust_mutex  [opt]    - Mutex operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_SEM
    printf("  rust_sem  [opt]      - Semaphore operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_MQ
    printf("  rust_mq              - Message queue operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_DL
    printf("  rust_dl              - Dynamic library operations demo\n");
    #endif
    #ifdef RUST_BENCH_TEST
    printf("  bench_test            - Benchmark test\n");
    #endif
    printf("  rust_test            - Run test suite\n");
    printf("  rust_help            - Show this help\n");
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_help, rust_help, Show Rust component help);

/* Component initialization */
static int rust_component_init(void)
{
    int ret = rust_init();
    if (ret == 0)
    {
        printf("Use 'rust_help' to see available commands\n");
    }
    return ret;
}
#ifdef RUST_INIT_COMPONENT
INIT_APP_EXPORT(rust_component_init);
#endif
