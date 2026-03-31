// Test de input GLFW para Termux-X11
#include <GLFW/glfw3.h>
#include <stdio.h>
#include <stdlib.h>

void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GLFW_TRUE);
        return;
    }
    
    const char* key_name = glfwGetKeyName(key, scancode);
    if (key_name) {
        printf("[KEY] %s - Action: %d - Mods: %d\n", key_name, action, mods);
    } else {
        printf("[KEY] Key %d - Action: %d - Mods: %d\n", key, action, mods);
    }
    fflush(stdout);
}

int main(void) {
    GLFWwindow* window;

    if (!glfwInit()) {
        fprintf(stderr, "Failed to initialize GLFW\n");
        return -1;
    }

    window = glfwCreateWindow(640, 480, "Test Input Termux-X11", NULL, NULL);
    if (!window) {
        fprintf(stderr, "Failed to create GLFW window\n");
        glfwTerminate();
        return -1;
    }

    glfwSetKeyCallback(window, key_callback);
    glfwMakeContextCurrent(window);

    printf("=== GLFW Input Test ===\n");
    printf("Window: 640x480\n");
    printf("Press ESC to exit\n");
    printf("Press any key to see events\n");
    printf("=======================\n\n");
    fflush(stdout);

    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        glfwSwapBuffers(window);
    }

    glfwTerminate();
    return 0;
}
