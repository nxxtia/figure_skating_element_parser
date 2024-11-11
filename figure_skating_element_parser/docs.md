# Figure Skating Element Parser Grammar

This documentation explains the rules and structure of the grammar used in the figure skating element parser.

## Rules

### WHITESPACE

This rule matches any whitespace characters (space, tab, newline).

```plaintext
WHITESPACE = _{ " " | "\t" | "\n" }

### DIGIT

This rule matches any numeric characters from 0 to 9.

```plaintext
DIGIT = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" }

### element

This main rule defines all possible figure skating elements that the parser can recognize. Each element type corresponds to a distinct category.

```plaintext
element = { jump | spin | step_sequence | death_spiral | pair_spin | twizzle | choreographic_element }

### jump

The rule for jumps, supporting various levels of difficulty, including single, double, triple, and quad jumps.

```plaintext
jump = { ("1" | "2" | "3" | "4") ~ jump_type ~ throw_jump? }

### jump_type: 
Defines the type of jump (e.g., Toeloop, Salchow, Loop, Flip, Lutz, Axel).

```plaintext
jump_type = { "T" | "S" | "Lo" | "F" | "Lz" | "A" }

### throw_jump:
Indicates a "throw" jump (used in pairs skating).

```plaintext
throw_jump = { "Th" }

### spin

The rule for spins, including standard spins, flying spins, and change-foot spins.

```plaintext
spin = { (spin_position ~ "Sp") | flying_spin | flying_change_foot_spin }

### spin_position:
Specifies the spin position (e.g., Upright, Camel, Layback, Sit).

```plaintext
spin_position = { "U" | "L" | "C" | "S" }

### step_sequence

The rule for step sequences. These can be standard or choreographic step sequences.

```plaintext
step_sequence = { "StSq" | "ChSq" }

### death_spiral

The rule for death spirals. This includes forward and backward positions.

```plaintext
death_spiral = { death_spiral_position ~ "Ds" }

### death_spiral_position:
Defines the death spiral position (e.g., Forward Inside, Backward Outside).

```plaintext
death_spiral_position = { "Fi" | "Bi" | "Fo" | "Bo" }

### pair_spin

The rule for spins in pairs skating.

```plaintext
pair_spin = { "PSp" | "PCoSp" }

### twizzle

The rule for twizzles (rotational elements).

```plaintext
twizzle = { "STw" }

### choreographic_element

The rule for choreographic elements, including choreographic lifts and spins.

```plaintext
choreographic_element = { "ChLi1" | "ChSp1" }
