# Motion Functions

The motion functions are used to move sprites around the screen. They can be used to create animations, move sprites to specific positions, and more. Here is a list of all the motion functions in Crust:

- `move(steps)`: Moves the sprite forward by the specified number of steps. The direction is determined by the sprite's current direction.
- `turn_cw(angle)`: Turns the sprite clockwise by the specified angle in degrees.
- `turn_ccw(angle)`: Turns the sprite counter-clockwise by the specified angle in degrees.
- `goto(x, y)`: Moves the sprite to the specified coordinates `(x, y)`. The coordinates are in World coordinates, not screen coordinates.
- `goto("mouse" | sprite)`: Moves the sprite to the position of the mouse or another sprite. The sprite can be specified by its name. Example: `goto("mouse")`, `goto("player")`.
- `glide(x, y, time)`: Glides the sprite to the specified coordinates `(x, y)` over the specified time in seconds. The glide is smooth and takes the specified time to complete.
- `glide(x, y, time, "linear" | "ease" | "ease-in" | "ease-out" | "ease-in-out")`: Glides the sprite to the specified coordinates `(x, y)` over the specified time in seconds with the specified easing function. The easing functions determine how the speed of the glide changes over time.
- `point(angle)`: Points the sprite in the specified direction in degrees. The angle is relative to the right side of the sprite.
- `point("mouse" | sprite)`: Points the sprite towards the mouse or another sprite. The sprite can be specified by its name. Example: `point("mouse")`, `point("player")`.
- `point(x, y)`: Points the sprite towards the specified coordinates `(x, y)`. The coordinates are in World coordinates, not screen coordinates.
- `change_x(steps)`: Changes the sprite's x-coordinate by the specified number of steps. Positive values move the sprite to the right, negative values move it to the left.
- `set_x(x)`: Sets the sprite's x-coordinate to the specified value. The value is in World coordinates, not screen coordinates.
- `change_y(steps)`: Changes the sprite's y-coordinate by the specified number of steps. Positive values move the sprite up, negative values move it down.
- `set_y(y)`: Sets the sprite's y-coordinate to the specified value. The value is in World coordinates, not screen coordinates.
- `edge_bounce(boolean)`: Makes the sprite bounce off the edges of the screen. If `boolean` is `true`, the sprite will bounce off the edges. If `false`, the sprite will not bounce off the edges.
- `rotation_style("all-around" | "left-right" | "dont-rotate")`: Sets the sprite's rotation style. The rotation style determines how the sprite rotates when it moves. Only visual rotation is affected, not the direction of movement.
    - `"all-around"`: The sprite can rotate in any direction.
    - `"left-right"`: The sprite can only rotate left and right.
    - `"dont-rotate"`: The sprite does not rotate at all.
- `direction()`: Returns the sprite's current direction in degrees. The direction is relative to the right side of the sprite.
- `x()`: Returns the sprite's current x-coordinate in World coordinates.
- `y()`: Returns the sprite's current y-coordinate in World coordinates.
