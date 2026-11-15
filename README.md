# Stapang

## Table of Contents

1. [What is Stapang?](#1-what-is-stapang)
2. [Getting Started](#2-getting-started)
   - [How to get Stapang](#21-how-to-get-stapang)
   - [Basic Syntax](#22-basic-syntax)
   - [Comments](#23-comments)
   - [Data Types](#24-data-types)
   - [Reserved Words](#25-reserved-words)
   - [Data Structures](#26-data-structures)
3. [Instructions](#3-instructions)
   - [Stack and Data Management](#31-stack-and-data-management)
   - [Variables and Arrays](#32-variables-and-arrays)
   - [Arithmetic Operators](#33-arithmetic-operators)
   - [Comparison Operators](#34-comparison-operators)
   - [Logical Operators](#35-logical-operators)
   - [Control Flow](#36-control-flow)
4. [Errors](#4-errors)
   - [Syntax Errors](#41-syntax-errors)
   - [Runtime Errors](#42-runtime-errors)
5. [Examples](#5-examples)

## 1. What is Stapang?

Stapang is an interpreted, strongly typed, stack-based programming language. All operations are performed through a stack, which acts as the central mechanism for handling and manipulating values.

## 2. Getting Started

## 2.1. How to get Stapang

To install Stapang, you first need to have Rust and Cargo installed.

Once you have installed them, execute the following command:

`cargo install --git https://github.com/robertosabau/Stapang`

This command downloads the source code from the repository, compiles it, and installs the Stapang executable on your computer.

Once the installation is complete, you can run Stapang from any directory, provided that Cargo's binary directory is included in your system's PATH.

Note: Stapang source files are plain text files, so any text editor can be used to create them. However, using the `.sp` extension is recommended to identify Stapang source files.

### 2.2. Basic Syntax

There are two important rules to consider regarding the syntax of Stapang.

The first rule is that every instruction must end with a `;`.

The second rule is that Stapang is whitespace-insensitive. This means that you can write all the instructions on one line or use multiple lines for one instruction.

### 2.3. Comments

Stapang supports single-line comments. To write one, use the `#` character. The interpreter ignores everything after it until the next newline (`\n`).

### 2.4. Data Types

The following data types can be used when declaring variables and arrays in Stapang:

#### Strings

This type represents character strings encoded in UTF-8.

To create a variable or an array of this type, you must use `str` as the `type_of_value_allowed`.

#### Float

This type represents double-precision floating-point numbers based on the IEEE 754-2008 standard.

To create a variable or an array of this type, you must use `float` as the `type_of_value_allowed`.

#### Int

This type represents a signed 64-bit integer.

To create a variable or an array of this type, you must use `int` as the `type_of_value_allowed`.

#### Uint

This type represents an unsigned integer whose size depends on the target architecture.

To create a variable or an array of this type, you must use `uint` as the `type_of_value_allowed`.

This is the only data type that can be used as an array index.

#### Boolean

This type represents a Boolean value that can only have one of two possible values: `true` or `false`.

To create a variable or an array of this type, you must use `bool` as the `type_of_value_allowed`.

#### Any

`any` is not a data type itself. Instead, it is a keyword that allows a variable or an array to store values of any supported data type.

To create a variable or an array of this type, you must use `any` as the `type_of_value_allowed`.

### 2.5. Reserved Words

In Stapang, the following words are reserved:

#### True

The word `true` is reserved to represent the Boolean value `true`.

#### False

The word `false` is reserved to represent the Boolean value `false`.

### 2.6. Data Structures

#### Variables

A variable is a named storage location used to hold a value.

When a variable is created, it is uninitialized and must be assigned a value before it can be read.

Variables are mutable.

#### Arrays

An array is an indexed collection of positions, each capable of holding a value.

Array indexing starts at `0`.

When an array is created, all its positions are uninitialized and must be assigned a value before they can be read.

Arrays are mutable.

## 3. Instructions

### 3.1. Stack and Data Management

#### Push

This instruction pushes a value onto the stack.

The syntax is as follows:
* `push value;`
* `push name_of_variable;`
* `push name_of_array[position_of_the_array];`

#### Pop

This instruction pops the value at the top of the stack and puts it inside a variable or an array position.

The syntax is as follows:
* `pop name_of_variable;`
* `pop name_of_array[position_of_the_array];`

#### Print

This instruction takes the value at the top of the stack and prints it to the standard output.

The syntax is as follows:
* `print;`

#### Read

This instruction allows you to read a value from the standard input and specify its expected data type.

The read instruction currently supports `str`, `int`, `float` and `uint` as input types.

The syntax is as follows:
* `read type_of_value_to_read;`

### 3.2. Variables and Arrays

#### Var

This instruction allows you to create a variable.

The syntax is as follows:
* `var name_of_variable:type_of_value_allowed;`

#### Arr

This instruction allows you to create an array with a specified number of elements.

The syntax is as follows:
* `arr name_of_array[number_of_positions]:type_of_value_allowed;`

### 3.3. Arithmetic Operators

Since all values are handled through the stack, which follows the LIFO (Last In, First Out) principle, `first_value_of_stack` refers to the value at the top of the stack when the instruction is executed. `second_value_of_stack` refers to the value immediately below it, which was pushed onto the stack before the first value.

#### Add

This instruction adds the top two values on the stack.
* `first_value_of_stack + second_value_of_stack`

The result is pushed onto the stack.

The syntax is as follows:
* `add;`

#### Sub

This instruction subtracts the top two values on the stack.
* `first_value_of_stack - second_value_of_stack`

The result is pushed onto the stack.

The syntax is as follows:
* `sub;`

#### Mult

This instruction multiplies the top two values on the stack.
* `first_value_of_stack * second_value_of_stack`

The result is pushed onto the stack.

The syntax is as follows:
* `mult;`

#### Div

This instruction divides the top two values on the stack.
* `first_value_of_stack / second_value_of_stack`

The result is pushed onto the stack.

The syntax is as follows:
* `div;`

#### Mod

This instruction calculates the remainder when dividing the top two values on the stack.
* `first_value_of_stack % second_value_of_stack`

The result is pushed onto the stack.

The syntax is as follows:
* `mod;`

### 3.4. Comparison Operators

Since all values are handled through the stack, which follows the LIFO (Last In, First Out) principle, `first_value_of_stack` refers to the value at the top of the stack when the instruction is executed. `second_value_of_stack` refers to the value immediately below it, which was pushed onto the stack before the first value.

#### Equal

This instruction takes the top two values from the stack and checks whether they are equal. Two values are considered equal only if they have the same type and the same value. For example, the number `5` and the string `"5"` are not considered equal.

The result is pushed onto the stack.

The syntax is as follows:
* `equal;`

#### Greater

This instruction takes the top two values from the stack and checks whether the first value is greater than the second.
* `first_value_of_stack > second_value_of_stack`

When comparing strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The result is pushed onto the stack.

The syntax is as follows:
* `greater;`

#### Less

This instruction takes the top two values from the stack and checks whether the first value is less than the second.
* `first_value_of_stack < second_value_of_stack`

When comparing strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The result is pushed onto the stack.

The syntax is as follows:
* `less;`

#### GreaterEqual

This instruction compares the top two values on the stack and checks whether the first value is greater than or equal to the second.
* `first_value_of_stack >= second_value_of_stack`

When comparing strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The result is pushed onto the stack.

The syntax is as follows:
* `greaterEqual;`

#### LessEqual

This instruction compares the top two values on the stack and checks whether the first value is less than or equal to the second.
* `first_value_of_stack <= second_value_of_stack`

When comparing strings , lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The result is pushed onto the stack.

The syntax is as follows:
* `lessEqual;`

### 3.5. Logical Operators

#### And

This instruction takes the top two values from the stack and checks whether both values are the Boolean value `true`.

The result is pushed onto the stack.

The syntax is as follows:
* `and;`

#### Or

This instruction takes the top two values from the stack and checks whether at least one of the values is the Boolean value `true`.

The result is pushed onto the stack.

The syntax is as follows:
* `or;`

#### Not

This instruction takes the value at the top of the stack and, if it is a Boolean value, returns its logical negation.

The result is pushed onto the stack.

The syntax is as follows:
* `not;`

### 3.6. Control Flow

#### If

This instruction takes the value at the top of the stack. If the value is the Boolean value `true`, the instructions inside the if block are executed. Otherwise, an optional else block can be executed if the value is the Boolean value `false`.

The syntax is as follows:
```
if{
  instructions
}
#Optional
else{
  instructions
}
```

#### While

This instruction takes the value at the top of the stack. If it is the Boolean value `true`, the instructions inside the while block are executed. Once the block finishes executing, the interpreter takes the value at the top of the stack again and checks whether it is `true`. This process continues as long as the condition evaluates to `true`.

The syntax is as follows:
```
while{
  instructions
}
```

## 4. Errors

### 4.1. Syntax Errors

Syntax errors occur while the interpreter is analyzing the code, before execution, due to invalid syntax.

#### Unterminated String Error

This error occurs when a string literal is not properly closed with the character `"`.

#### Invalid Character Error

This error occurs when a character that is not allowed in the language syntax is detected. For example, '$' is not a valid character.

#### Invalid Identifier Error

This error occurs when an identifier used for a variable or an array contains an invalid character.

#### Keyword Expected Error

This error occurs when the interpreter expects an instruction to begin with a keyword but does not find a valid one.

#### Value Expected Error

This error occurs when an instruction requires a value, but no value is provided. This includes explicit values and references to variables or arrays.

#### Semicolon Expected Error

This error occurs when an instruction is not terminated with a `;`.

#### Opening Bracket Expected Error

This error occurs when an instruction, such as `arr`, expects an opening bracket `[` but does not find one.

#### Closing Bracket Expected Error

This error occurs when an instruction expects a closing bracket `]` but does not find one.

#### Number Expected Error

This error occurs when an instruction expects a numeric value but does not find one.

#### Opening Brace Expected Error

This error occurs when an instruction expects an opening brace `{` but does not find one.

#### Closing Brace Expected Error

This error occurs when an instruction expects a closing brace `}` but does not find one.

#### Colon Expected Error

This error occurs when an instruction expects a colon `:` but does not find one.

#### Type Expected Error

This error occurs when an instruction expects a data type but does not find one.

### 4.2. Runtime Errors

Runtime errors occur while the interpreter is executing instructions due to problems that arise during execution.

#### Boolean Expected Error

This error occurs when an `if` or `while` instruction expects a Boolean value at the top of the stack but finds a value of another type.

#### Stack Empty Error

This error occurs when an instruction tries to retrieve a value from the stack, but the stack is empty.

#### Variable Not Defined Error

This error occurs when an instruction tries to access a variable that has not been defined.

#### Array Not Defined Error

This error occurs when an instruction tries to access an array that has not been defined.

#### Internal Error

This error occurs when, for any reason, the interpreter reaches a part of the code that should never be executed.

If you encounter this error, please report it and describe the steps that caused it.

#### Out of Bounds Error

This error occurs when an attempt is made to access an array position outside the array's valid index range.

#### Invalid Type Error

This error occurs when an operation is performed using a value whose type does not support that operation or is incompatible with the type of the other value.

#### Division by Zero Error

This error occurs when a division or modulo operation attempts to divide by `0`.

#### Read Error

This error occurs when the standard input stream (`stdin`) is closed, unavailable, or cannot provide input.

#### Numeric Conversion Error

This error occurs when an attempt is made to convert an invalid string into a numeric type. This usually happens when the read instruction receives input that cannot be converted to the requested numeric type.

#### Invalid Value Error

This error occurs when an attempt is made to assign a value to a variable or an array position that is incompatible with its declared type. For example, assigning a Boolean value to a variable declared as `str`.

#### Variable Not Initialized Error

This error occurs when an attempt is made to retrieve the value of a variable that has not been initialized.

#### Array Position Not Initialized Error

This error occurs when an attempt is made to retrieve the value of an array position that has not been initialized.

## 5. Examples

Here are some examples of common programming problems and possible solutions implemented in Stapang.

### Hello World

In this example, we are going to perform the classic task of printing `"Hello World"`:

**Code:**
```stapang
push "Hello World"; # We push the value we want to print onto the top of the stack
print; # We print the value at the top of the stack
```

**Output:**
```
Hello World
```

### Basic Arithmetic

In this example, we are going to read two integer numbers and perform the four basic arithmetic operations: addition, subtraction, multiplication, and division.

**Code:**
```stapang
# Declaration of variables to store the numbers
var num1 : int;
var num2 : int;

# Reading the first number
push "Enter a number: ";
print;
read int;
pop num1;

# Reading the second number
push "Enter another number: ";
print;
read int;
pop num2;

# Calculate and print the addition (num1+num2)
push num2;
push num1;
add;
push "Addition: ";
print;
print;

# Printing a line break for better readability
push "\n";
print;

# Calculate and print the subtraction (num1-num2)
push num2;
push num1;
sub;
push "Subtraction: ";
print;
print;

# Printing a line break for better readability
push "\n";
print;

# Calculate and print the multiplication (num1*num2)
push num2;
push num1;
mult;
push "Multiplication: ";
print;
print;

# Printing a line break for better readability
push "\n";
print;

# Calculate and print the division (num1/num2)
push num2;
push num1;
div;
push "Division: ";
print;
print;
```

**Example execution:**
```
Enter a number: 4
Enter another number: 2
Addition: 6
Subtraction: 2
Multiplication: 8
Division: 2
```

### Variable Manipulation

In this example, we are going to read two numbers, store them in two variables, and swap their values using a third variable. Finally, we will print the resulting values.

**Code:**
```stapang
# Declaration of variables to store the numbers
var num1 : int;
var num2 : int;
var temp : int;

# Reading the first number
push "Enter a number: ";
print;
read int;
pop num1;

# Reading the second number
push "Enter another number: ";
print;
read int;
pop num2;

# Printing the initial values
push "\n";
push num1;
push "num1 = ";
print;
print;
print;
push "\n";
push num2;
push "num2 = ";
print;
print;
print;

# Swapping the values
push num1;
pop temp;
push num2;
pop num1;
push temp;
pop num2;

# Printing the resulting values
push "\n";
push num1;
push "num1 = ";
print;
print;
print;
push "\n";
push num2;
push "num2 = ";
print;
print;
print;
```

**Example execution:**
```
Enter a number: 2
Enter another number: 4
num1 = 2
num2 = 4
num1 = 4
num2 = 2
```

### Conditional Statements

In this example, we are going to ask the user for an integer and indicate whether the number is positive, negative, or zero.

**Code:**
```stapang
# Declaration of a variable to store the number
var num : int;

# Reading the number
push "Enter a number: ";
print;
read int;
pop num;

# Checking if the number is greater than 0 (number>0)
push 0;
push num;
greater;
if {
    push "The number is positive";
    print;
}
else{
    # Checking if the number is less than 0 (number<0)
    push 0;
    push num;
    less;
    if{
        push "The number is negative";
        print;
    }
    else{
        push "The number is 0";
        print;
    }
}
```

**Example execution 1:**
```
Enter a number: 9
The number is positive
```

**Example execution 2:**
```
Enter a number: -5
The number is negative
```

**Example execution 3:**
```
Enter a number: 0
The number is 0
```

### Loops and Arrays

In this example, we are going to initialize an array with five values and use a loop to print all of them.

**Code:**
```stapang
# Array declaration
arr numbers[5] : int;

# Declaration of a variable to store the array index
var index: uint;

# Array initialization (not the recommended way to do it: {0,-53,10,20,4})
push 4;
push 20;
push 10;
push -53;
push 0;
pop numbers[0];
pop numbers[1];
pop numbers[2];
pop numbers[3];
pop numbers[4];

# Initialize the index
push 0;
pop index;

push true; # Push true to enter the first iteration
while {
    # Printing the value at the current index
    push "\n";
    push numbers[index];
    print;
    print;
    # Incrementing the index
    push 1;
    push index;
    add;
    pop index;
    # Checking if the index is less than or equal to 4
    push 4;
    push index;
    lessEqual;
}
```

**Output:**
```
0
-53
10
20
4
```
