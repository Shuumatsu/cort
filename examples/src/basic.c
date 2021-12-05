#include <stdio.h>
#include <stdlib.h>
#include "cort.h"

volatile int a_started, b_started, c_started;
volatile int a_n, b_n, c_n;

void thread_a(void) {
    printf("thread_a started\n");
    a_started = 1;
    while (b_started == 0 || c_started == 0) {
        printf("thread_a not ready\n");
        yield_now();
    }

    for (int i = 0; i < 100; i++) {
        printf("thread_a %d\n", i);
        a_n += 1;
        yield_now();
    }
    printf("thread_a: exit after %d\n", a_n);

    t_return();
}

void thread_b(void) {
    printf("thread_b started\n");
    b_started = 1;
    while (a_started == 0 || c_started == 0) yield_now();

    for (int i = 0; i < 100; i++) {
        printf("thread_b %d\n", i);
        a_n += 1;
        yield_now();
    }
    printf("thread_b: exit after %d\n", a_n);

    t_return();
}

void thread_c(void) {
    printf("thread_c started\n");
    c_started = 1;
    while (b_started == 0 || b_started == 0) yield_now();

    for (int i = 0; i < 100; i++) {
        printf("thread_c %d\n", i);
        a_n += 1;
        yield_now();
    }
    printf("thread_c: exit after %d\n", a_n);

    t_return();
}

int main(int argc, char* argv[]) {
    a_started = b_started = c_started = 0;
    a_n = b_n = c_n = 0;
    spawn(thread_a);
    spawn(thread_b);
    spawn(thread_c);
    schedule();
    exit(0);
}
