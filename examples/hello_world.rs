//! Ported from http://www.glfw.org/documentation.html

extern crate glfw_ffi;

use glfw_ffi::*;
use std::ptr;

fn main() {
    unsafe {
        /* Initialize the library */
        if glfwInit() == 0 {
            return;
        }

        /* Create a windowed mode window and its OpenGL context */
        let window = glfwCreateWindow(
            640,
            480,
            b"Hello World\0".as_ptr() as _,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if window.is_null() {
            glfwTerminate();
            return;
        }

        /* Make the window's context current */
        glfwMakeContextCurrent(window);

        /* Loop until the user closes the window */
        while glfwWindowShouldClose(window) == 0 {
            /* Render here */

            /* Swap front and back buffers */
            glfwSwapBuffers(window);

            /* Poll for and process events */
            glfwPollEvents();
        }

        glfwTerminate();
    }
}
