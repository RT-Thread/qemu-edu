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
#endif

/* string_demo module */
#ifdef RUST_EXAMPLE_STRING
extern size_t rust_strlen_demo(const char *s);
extern int rust_strcmp_demo(const char *s1, const char *s2);
extern void rust_strcpy_demo(void);
extern void rust_strcat_demo(void);
extern int rust_strstr_demo(const char *haystack, const char *needle);
extern void rust_string_demo_all(void);
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
extern void rust_thread_demo_all(void);
#endif

/* vec_demo module */
#ifdef RUST_EXAMPLE_VEC
extern void rust_vec_demo(void);
extern void rust_vec_demo_all(void);
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

/* String command: strlen */
#ifdef RUST_EXAMPLE_STRING
static int cmd_rust_strlen(int argc, char **argv)
{
    if (argc < 2)
    {
        printf("Usage: rust_strlen <string>\n");
        return -1;
    }
    
    size_t len = rust_strlen_demo(argv[1]);
    printf("String '%s' length: %zu\n", argv[1], len);
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_strlen, rust_strlen, Calculate string length);

/* String command: strcmp */
static int cmd_rust_strcmp(int argc, char **argv)
{
    if (argc < 3)
    {
        printf("Usage: rust_strcmp <string1> <string2>\n");
        return -1;
    }
    
    int result = rust_strcmp_demo(argv[1], argv[2]);
    printf("strcmp('%s', '%s') = %d\n", argv[1], argv[2], result);
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_strcmp, rust_strcmp, Compare two strings);

/* String demonstration */
static int cmd_rust_string(int argc, char **argv)
{
    rust_string_demo_all();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_string, rust_string, Demonstrate string operations);
#endif /* RUST_EXAMPLE_STRING */

/* Memory demonstration */
#ifdef RUST_EXAMPLE_MEMORY
static int cmd_rust_memory(int argc, char **argv)
{
    rust_memory_demo_all();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_memory, rust_memory, Demonstrate memory operations);
#endif

/* Vector demonstration */
#ifdef RUST_EXAMPLE_VEC
static int cmd_rust_vec(int argc, char **argv)
{
    rust_vec_demo();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_vec, rust_vec, Demonstrate vector operations);
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
    else
    {
        printf("Usage: rust_thread [create|self|sleep|wrapper]\n");
        printf("  Without arguments: run all demos\n");
        printf("  create  - demonstrate thread creation\n");
        printf("  self    - show current thread info\n");
        printf("  sleep   - demonstrate thread sleep\n");
        printf("  wrapper - demonstrate Rust thread wrapper\n");
    }
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_thread, rust_thread, RT-Thread operations demo);
#endif

/* Printf demonstration */
#ifdef RUST_EXAMPLE_PRINTF
static int cmd_rust_printf(int argc, char **argv)
{
    rust_printf_demo();
    rust_sprintf_demo();
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_printf, rust_printf, Demonstrate printf operations);
#endif

/* Comprehensive test command */
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
    
    /* 3. String test */
    printf("\n3. String Test:\n");
    #ifdef RUST_EXAMPLE_STRING
    const char *test_str = "RT-Thread";
    printf("   strlen(\"%s\") = %zu\n", test_str, rust_strlen_demo(test_str));
    #else
    printf("   (string example disabled)\n");
    #endif
    
    /* 4. Arithmetic test */
    printf("\n4. Arithmetic Test:\n");
    #ifdef RUST_EXAMPLE_MEMORY
    printf("   42 + 58 = %d\n", rust_add(42, 58));
    printf("   10 * 20 = %d\n", rust_multiply(10, 20));
    #else
    printf("   (memory/arithmetic example disabled)\n");
    #endif
    
    /* 5. Memory test */
    printf("\n5. Memory Test:\n");
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
    #ifdef RUST_EXAMPLE_STRING
    printf("  rust_strlen <str>    - Get string length\n");
    printf("  rust_strcmp <s1> <s2> - Compare strings\n");
    printf("  rust_string          - String operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_PRINTF
    printf("  rust_printf          - Printf operations demo\n");
    #endif
    #ifdef RUST_EXAMPLE_THREAD
    printf("  rust_thread [opt]    - Thread operations demo\n");
    #endif
    printf("  rust_test            - Run test suite\n");
    printf("  rust_help            - Show this help\n");
    #ifdef RUST_EXAMPLE_VEC
    printf("  rust_vec             - Vector operations demo\n");
    #endif
    return 0;
}
MSH_CMD_EXPORT_ALIAS(cmd_rust_help, rust_help, Show Rust component help);

/* Component initialization */
static int rust_component_init(void)
{
    int ret = rust_init();
    if (ret == 0)
    {
        printf("Rust component initialized (modular version)\n");
        printf("Use 'rust_help' to see available commands\n");
    }
    return ret;
}
#ifdef RUST_INIT_COMPONENT
INIT_APP_EXPORT(rust_component_init);
#endif
