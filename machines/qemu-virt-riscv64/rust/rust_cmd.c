/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-15     foxglove     1.0 version
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