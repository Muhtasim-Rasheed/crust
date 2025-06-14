# Misc

- `args()`
- `print(values...)`
- `input(prompt)`
- `time()`
- `abs(num)`
- `sqrt(num)`
- `sin(num)`
- `cos(num)`
- `tan(num)`
- `asin(num)`
- `acos(num)`
- `atan(num)`
- `lerp(a, b, t)`
- `property_of(sprite, property)`
- `to_rad(deg)`
- `to_deg(rad)`
- `clamp(value, min, max)`
- `len(list | string)`
- `random(min, max)`
- `distance(x1, y1, x2, y2)`
- `distance_to(x, y)`
- `export(content)`
- `export(content, path)`
- `import(path)`
- `import_binary(path)`
- `parse_image(binary)`
- `screenshot()`
- `screenshot(path)`
- `push(list)`
- `pop(list)`
- `insert(list, index, value)`
- `remove(list, index)`
- `extend(list1, list2)`
- `range(end)`
- `range(start, end)`
- `range(start, end, step)`
- `whoami()`
- `clone_id()`
- `frame()`
- `delta_time()`

# Motion [DONE]

- [x] `move(steps)`
- [x] `turn_cw(angle)`
- [x] `turn_ccw(angle)`
- [x] `goto(x, y)`
- [x] `goto(mouse | sprite)`
- [x] `glide(x, y, time)`
- [x] `glide(x, y, time, linear | ease | ease-in | ease-out | ease-in-out)`
- [x] `point(angle)`
- [x] `point(mouse | sprite)`
- [x] `point(x, y)`
- [x] `change_x(steps)`
- [x] `set_x(x)`
- [x] `change_y(steps)`
- [x] `set_y(y)`
- [x] `edge_bounce(true | false)`
- [x] `rotation_style()`
- [x] `direction()`
- [x] `x()`
- [x] `y()`

# Looks [DONE]

- [x] `say(string, time)`
- [x] `say(string)`
- [x] `think(string, time)`
- [x] `think(string)`
- [x] `switch_costume(costume)`
- [x] `next_costume()`
- [x] `previous_costume()`
- [x] `switch_backdrop(backdrop)`
- [x] `next_backdrop()`
- [x] `previous_backdrop()`
- [x] `change_size(increment)`
- [x] `set_size(size)`
- [x] `change_effect(effect, increment)`
- [x] `set_effect(effect, value)`
- [x] `clear_effects()`
- [x] `clear_effect(effect)`
- [x] `go_to_layer(layer)`
- [x] `go_by_layers(forward | backward, steps)`
- [x] `costume()`
- [x] `backdrop()`
- [x] `size()`
- [x] `scale()`
- [x] `bounds()`
- [x] `layer()`
- [x] `effect(effect)`

# Sound

- [x] `play_sound(sound, stop-other-sounds)`
- [ ] `play_sound_until_done(sound)`
- [x] `stop_all_sounds()`
- [x] `stop_sound(sound)`
- [x] `change_sound_effect(effect, increment)`
- [x] `set_sound_effect(effect, value)`
- [x] `sound_effect(effect)`

# Events

- [x] `setup {}`
- [x] `update {}`
- [x] `key_down(key)`
- [x] `key_pressed(key)`
- [x] `key_release(key)`
- [x] `sprite_clicked()`
- [x] `is_backdrop(backdrop)`
- [x] `when bool {}`
- [x] `when broadcast {}`
- [x] `broadcast(broadcast)`
- [ ] `broadcast_and_wait(broadcast)`

# Control [DONE]

- [x] `wait(sec)`
- [x] `repeat x {}`
- [x] `if bool {}`
- [x] `if bool {} else {}`
- [x] `if bool {} else if bool {}`
- [x] `while bool {}`
- [x] `for i in iterable {}`
- [x] `stop(all | this | script | other-scripts | other-sprites-and-scripts)`
- [x] `clone_setup {}`
- [x] `clone_update {}`
- [x] `clone()`
- [x] `delete_clone(cloneid)`
- [x] `delete_clone()`
- [x] `skip_further_execution_if(bool)`

# Drawing [DONE]

- [x] `set_color(r, g, b)`
- [x] `change_r(increment)`
- [x] `change_g(increment)`
- [x] `change_b(increment)`
- [x] `line(x1, y1, x2, y2, thickness)`
- [x] `rect(x1, y1, x2, y2)`
- [x] `hrect(x1, y1, x2, y2, thickness)`
- [x] `circle(x, y, radius)`
- [x] `hcircle(x, y, radius, thickness)`
- [x] `ellipse(x, y, width, height)`
- [x] `ellipse(x, y, width, height, rotation)`
- [x] `hellipse(x, y, width, height, thickness)`
- [x] `hellipse(x, y, width, height, rotation, thickness)`
- [x] `polygon(x1, y1, ..., xN, yN)`
- [x] `hpolygon(thickness, x1, y1, ..., xN, yN)`
- [x] `textured_quad(parse_image_result, x1, y1, x2, y2, x3, y3, x4, y4)`
- [x] `stamp()`
- [x] `clear_all_stamps()`
- [x] `r()`
- [x] `g()`
- [x] `b()`

# Window

- [x] `set_window_width(width)`
- [x] `set_window_height(height)`
- [x] `set_window_size(width, height)`
- [x] `set_window_state(state)`
    - [x] `normal`
    - [ ] `windowed-fullscreen`
    - [x] `fullscreen`
    - [ ] `borderless-windowed`
    - [ ] `minimized`
- [x] `set_window_x(x)`
- [x] `set_window_y(y)`
- [x] `set_window_position(x, y)`
- [x] `pointer_grab(bool)`
- [x] `window_width()`
- [x] `window_height()`
- [ ] `screen_width()`
- [ ] `screen_height()`
