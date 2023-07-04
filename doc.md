# Sick as
`sick` is an assemblish language. Which means it has very basic instructions,
each with a 3 letter name. I made it just to test `nom` ngl.

Comments use an octothorpe (`#`). Memory addresses are numbers prefixed by `m`.
If an instruction has `->`, it means a value is being saved to that memory
address.

You have 256 bytes of memory, but `m0` is the instruction pointer. Good luck.

## Instructions

The possible instructions are: (where `a` is either a number literal or memory address, and `m` a memory address)
- `set a -> m`
- `and a, a -> m`
- `xor a, a -> m`
- `not a, a -> m`
- `add a, a -> m`
- `sub a, a -> m`
- `out a`
- `num a`
- `cin -> m`
- `nin -> m`
- `bak a, a`
- `fwd a, a`
- `bye a`

There is no `or` instruction because it is not 3 letters long. You can make do
with the knowledge that `not (not x and not y) = x or y`.

### set
Sets that memory address to the given value

```sick
set 10 -> m1
num m1  # 10
```

### and
Runs a **bitwise** `and` on the given values, result is stored in the given address.

For reference,

| a | b | a and b |
|:-:|:-:|:-------:|
| 0 | 0 |    0    |
| 1 | 0 |    0    |
| 0 | 1 |    0    |
| 1 | 1 |    1    |

``` sick
set 97 -> m1
out m1  # a

and m1, 95 -> m2
out m2  # A
```

### xor
Runs a **bitwise** `xor` on the given values, result is stored in the given address.

For reference,

| a | b | a xor b |
|:-:|:-:|:-------:|
| 0 | 0 |    0    |
| 1 | 0 |    1    |
| 0 | 1 |    1    |
| 1 | 1 |    0    |

```sick
out 97  # 'a'
out 58  # ':'
out 32  # ' '
nin -> m1


out 98  # 'b'
out 58  # ':'
out 32  # ' '
nin -> m2

out 97  # 'a'

xor m1, m2 -> m10

fwd 3, m10
out 61  # '='   # 1

fwd 2, 0        # 2
out 33  # '!'   # 3  1

out 61  # '='   #    2

out 98  # 'b'
out 10  # '\n'
```

### not
Runs a **boolean** `not` on the given value. 0 gets turned into 1, any non-zero value gets turned into 0.

For reference,

| a | not a |
|:-:|:-----:|
| 0 |   1   |
| 1 |   0   |

```sick
set 45 -> m1
not m1 -> m2
num m2  # 0

set 0 -> m1
not m1 -> m2
num m2  # 1
```

### add
Adds the two values together, and sets that memory address.

```sick
add 10, 20 -> m1
num m1  # 30
```

### sub
Sets that memory address to the first value minus the second value.

```sick
sub 10, 1 -> m1
num m1  # 9

sub 0, 1 -> m2
num m2  # 255
```

### out
Prints the corresponding ASCII character of the given value, without a newline.

```sick
out 65  # A
out 10  # \n
```

### num
Prints the given value as a number, without a newline.

```sick 
num 14  # 14
out 10  # \n

set 66 -> m1
num m1  # B
out 10  # \n
```

### cin
Reads one character from stdin and saves it to the given address
```
cin -> m1  # Input: h
num m1     # 104
```

### nin
Reads a number from stdin.

```
nin -> m1  # Input: 105
num m1     # 105
out m1     # i
```

### bak
Like `jz` in x86 assembly. It moves back the number of instructions given by
the first value, if the second value is not equal to 0.

```sick
set 10 -> m1

out 65  # A
sub m1, 1 -> m1
bak 2, m1

out 10 # \n
# Final output: AAAAAAAAAA
```

### fwd
Like `bak`, but forward.

```sick
# Print "y/n: " and ask for input
out 121  # 'y'
out 47   # '/'
out 110  # 'n'
out 58   # ':'
out 32   # ' '
cin -> m1

# Check if they gave 'y' or 'n'
sub m1, 121 -> m10
sub m1, 110 -> m11

# Make sure that at least one is 0 (equal)
and m10, m11 -> m2

fwd 3, m2
out 87  # W
bye 0

out 76
```

### bye
Exit with the given return code
```
bye 0    # Exit with code 0
bye m31  # Exit with code in address 31
#^^^^^^ This line here is unreachable
```

## Example code

```sick
# Sets memory 1 to address 1
set 1 -> m1

# Add 1 and 2, save to address 2
add 1, 2 -> m2

# Add 10 and 7, save to address 3
sub 10, 7 -> m3

# Print A
out 65
# Print 64
num 64

# Print value in address 65 as a char
out m65
# Print value in address 64 as a number
num m64

# Take in a character from stdin, save to address 4
cin -> m4

# Take in a number from stdin, save to address 5
nin -> m5

# Go back 7 instructions if address 5 is not 0
bak 7, m5

# Go forward 7 instructions if address 50 is not 0
fwd 7, m50

out 104

# This does nothing
nop
nop
nop
nop
nop
out 105

out 110
out 111

# Exit with this return code
bye 0
```

## Possible future features

`req`: Executes another `sick` program.
`sys`: Performs a syscall (too useful?)
`swp`: Swaps the direction of the instruction pointer. Would make `bak` go forward and `fwd` go backward.

Also support for `'a'` chars rather than typing out the code, would be useful but I can't be bothered tbh.
