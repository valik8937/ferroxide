#include <stdio.h>
#include <dlfcn.h>

// EGL_DEFAULT_DISPLAY зазвичай дорівнює NULL (0).
// Ми визначаємо його тут для простоти.
#define EGL_DEFAULT_DISPLAY ((void *)0)

int main() {
    printf("--- Test EGL started ---\n");

    // Завантажуємо стандартну системну бібліотеку для графіки EGL
    void *handle = dlopen("libEGL.so", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "Error: Cannot open libEGL.so. Is it installed?\n");
        return 1;
    }
    printf("Successfully opened libEGL.so\n");

    // Шукаємо функцію 'eglGetDisplay'. ЦЕЙ РЯДОК МАЄ ПЕРЕХОПИТИ НАШ dlsym ХУК.
    void* eglGetDisplay_ptr = dlsym(handle, "eglGetDisplay");

    if (eglGetDisplay_ptr) {
        printf("Successfully got a pointer for 'eglGetDisplay'.\n");

        // Визначаємо тип для нашої функції, щоб її можна було викликати
        typedef void* (*eglGetDisplay_t)(void* display_id);
        eglGetDisplay_t func = (eglGetDisplay_t)eglGetDisplay_ptr;

        // ВИКЛИКАЄМО ФУНКЦІЮ. ЦЕЙ РЯДОК МАЄ ЗАПУСТИТИ НАШУ ОБГОРТКУ.
        printf("Now calling the function...\n");
        void* display = func(EGL_DEFAULT_DISPLAY);

        printf("...function returned. Result: %p\n", display);
        if (display != NULL) {
            printf("SUCCESS! Got a valid EGL display.\n");
        } else {
            fprintf(stderr, "FAILURE! Did not get a valid EGL display.\n");
        }

    } else {
        fprintf(stderr, "Error: Failed to find 'eglGetDisplay' symbol.\n");
    }

    dlclose(handle);
    printf("--- Test EGL finished ---\n");
    return 0;
}