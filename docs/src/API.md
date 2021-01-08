# API

## GFX

### Colors
| Color       | Number |
|-------------|--------|
| Black       | 0      |
| Dark Blue   | 1      |
| Magenta     | 2      |
| Dark Green  | 3      |
| Bronze      | 4      |
| Grey        | 5      |
| Silver      | 6      |
| Off White   | 7      |
| Red         | 8      |
| Gold        | 9      |
| Yellow      | 10     |
| Green       | 11     |
| Sky Blue    | 12     |
| Pale Purple | 13     |
| Pink        | 14     |
| Peach       | 15     |

### `rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32)`
Draws a rectangle with points (`x0`, `y0`), (`x1`, `y1`)

### `rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32)`
Draws and fills a rectangle with points (`x0`, `y0`), (`x1`, `y1`)

### `pset(x: i32, y: i32, color: i32)`
Sets a pixel at (`x`, `y`) to `color`

### `print(string: *const c_char, x: i32, y: i32, col: i32)`
Prints `string` (null-terminated ASCII char pointer) to the screen at (`x`, `y`).  Font is 8x8

### `printh(string: *const c_char)`
Prints `string` (null-terminated ASCII char pointer) to the console.

## Input 

### Players
| Player | Number |
|--------|--------|
| One    | 1      |
| Two    | 2      |

### Buttons
| Button | Number |
|--------|--------|
| Left   | 0      |
| Right  | 1      |
| Up     | 2      |
| Down   | 3      |
| O      | 4      |
| X      | 5      |

### `btn(button: i32, player: i32)` ➜ `bool`
Returns true if `button` is pressed, false if not.

### `btnp(button: i32, player: i32)` ➜ `bool`
Returns true if `button` was pressed this frame, false if not.

### `key()` ➜ `i32`
Returns scancode of keyboard key pressed during last frame or 0 if there were no more keys pressed last frame to process. Calling this function will pop this key off the bottom of the list and if you call it again you will get the next key pressed.

## Misc

### `exit()` ➜ `!`
Exits