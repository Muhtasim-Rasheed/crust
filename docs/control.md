Most of the control functions are block statements and explained in the [Crust Language](crust-language.md) page. Here is a list of all the control functions in Crust:

- `wait(time)`: Waits for the specified time in seconds before continuing to the next block.
- `clone()`: Creates a clone of the sprite. The clone will execute the `clone_setup` and `clone_update` blocks.
- `delete_clone()`: Deletes the current clone. This function is typically called within the `clone_update` block to remove the clone when it is no longer needed.
- `delete_clone(cloneid)`: Deletes the specified clone. The `cloneid` is the ID of the clone to delete, which starts from 1 and increments for each clone created.

This page is so short unlike others...
