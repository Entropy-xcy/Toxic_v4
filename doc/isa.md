# Toxic Instruction Set Architecture

## Configurable Bus Width
Toxic's bus can be configured to have width of 4-bits, 8-bits, 12-bits or 16-bits.
**Bus width is denoted as `bw` in the following sections.**

## Address Space
* Instruction Memory: `[0...2^bw-1]`
  * Interrupt Handling: `[0...(2^bw)/8-1]`
  * Main Program: `[(2^bw)/4...2^bw-1]`
* Data Memory: `[0...2^bw-1]`
  * Heap: Grows from low address to high address.
  * Stack: Grows from high address to low address.
  * IO space: `[(2^bw)/8*3...(2^bw)/2]`

## Registers
* `sp`: stack pointer that points to the top-of-stack (tos) in the data memory.
* `pt`: general purpose pointer that points to any location in the data memory.
* `pc`: program counter that points to the next instruction to be executed. 

## Instructions
### Push
#### P0
Push 0 to stack.
```c
push(0)
```
#### P1
Push 1 to stack.
```c
push(1)
```

### Arithmetic 
#### NOT
```c
push(!pop())
```

#### LS
```c
push(pop() << 1)
```

#### RS
```c
push(pop() >> 1)
```

#### AND
```c
push(pop() & pop())
```

#### OR
```c
push(pop() | pop())
```

#### ADD
```c
push(pop() + pop())
```

#### CMP
```c
int tos = pop();
int ntos = pop();
int carry = (tos + ntos) > 15;
int gt = tos > ntos;
int lt = tos < ntos;
int eq = tos == ntos;
push((carry << 3) + (lt << 2) + (gt << 1) + eq);
```

### Pointers
#### ADR
```c
pt = pt << 4 + pop();
```

#### OFF
```c
pt += pop();
```

#### SP
```c
pt = sp;
```
#### PC
```c
pt = pc;
```

### Memory
#### PUT
```c
mem[pt] = pop();
```

#### GET
```c
push(mem[pt]);
```

### Branch
#### BR
```c
int tos = pop();
push(tos); // Does not Change Stack
pc = tos ? pt : pc;
```