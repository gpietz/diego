# OpenGL Groups Overview

----

### SETUP

* glClearColor(r, g, b, a) – Sets the background color.
* glViewport(x, y, width, height) – Defines the visible area of the window.
* glEnable(GL_DEPTH_TEST) – Enables depth testing.
* glDisable(GL_BLEND) – Disables blending (used for transparency).

### BUFFER 

* glGenBuffers(n, buffers) – Creates buffers.
* glBindBuffer(target, buffer) – Binds a buffer to a target (e.g., GL_ARRAY_BUFFER).
* glBufferData(target, size, data, usage) – Stores data in the buffer.
* glDeleteBuffers(n, buffers) – Deletes buffers.
* glIsBuffer - Checks whether a given name (ID) corresponds to a valid buffer object. 

### VAO

* glGenVertexArrays(n, arrays) – Creates VAOs.
* glBindVertexArray(array) – Binds a VAO.
* glVertexAttribPointer(...) – Defines how vertex data is interpreted.
* glEnableVertexAttribArray(index) –  Enables an attribute.
* glDisableVertexAttribArray(index) – Disables an attribute.
* glDeleteVertexArrays(n, arrays) – Deletes VAOs.

### SHADER

* glCreateShader(type) –  Creates a shader.
* glShaderSource(shader, source) – Sets the source code for a shader.
* glCompileShader(shader) – Compiles the shader.
* glCreateProgram() – Creates a shader program.
* glAttachShader(program, shader) – Attaches a shader to the program.
* glLinkProgram(program) – Links the shader program.
* glUseProgram(program) – Activates the shader program.
* glDeleteShader(shader) – Deletes the shader after linking.

### TEXTURES

* glGenTextures(n, textures) – Creates textures.
* glBindTexture(target, texture) – Binds a texture (e.g., GL_TEXTURE_2D).
* glTexImage2D(target, ...) –  Loads image data into a 2D texture.
* glTexParameteri(target, pname, param) – Sets parameters for the texture (e.g., filtering or wrapping modes).
* glDeleteTextures(n, textures) – Deletes textures.

### RENDERING

* glDrawArrays(mode, first, count) – Draws primitive shapes using vertex data.
* glDrawElements(mode, count, type, indices) – Draws using index data.
* glClear(mask) –  Clears the buffer (e.g., GL_COLOR_BUFFER_BIT and GL_DEPTH_BUFFER_BIT).

### FRAMEBUFFER

* glGenFramebuffers(n, framebuffers) –  Creates framebuffers.
* glBindFramebuffer(target, framebuffer) – Binds a framebuffer.
* glFramebufferTexture2D(target, attachment, textarget, texture, level) – Binds a texture to the framebuffer.
* glCheckFramebufferStatus(target) – Checks if the framebuffer is complete.
* glDeleteFramebuffers(n, framebuffers) – Löscht Framebuffers.

### TRANSFORMATION

* glUniformMatrix4fv(location, count, transpose, value) 
* glUniform1i(location, v0)

### STATE

* glGetInteger(enum, *data) - Retrieves various OpenGL states and limits (e.g., max textures, max vertex attributes)
