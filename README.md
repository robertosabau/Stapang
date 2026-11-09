# Stapang


## Table of Contents

1. [What is Stapang?](#1-what-is-stapang)
2. [Getting Started](#2-getting-started)
   - [Basic Syntax](#21-basic-syntax)
   - [Comments](#22-comments)
   - [Data Types](#23-data-types)
   - [Reserved Words](#24-reserved-words)
   - [Data Structures](#25-data-structures)
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
## 2.1. Basic Syntax

There are two important rules to consider regarding the syntax of the programming language.

The first one is that every instruction must end with a `;`.

The second one is that this programming language is whitespace-insensitive. This means that you can write all the instructions on one line or use one line for each part of the instruction.
### 2.2. Comments

There is only one type of comment: single-line comments. To write a comment, you must use the character `#`, which makes the interpreter ignore everything after it until the first `\n`.
### 2.3. Data Types

The following types are available for variables and arrays in Stapang:

#### 2.3.1. Strings

This type represents character strings encoded in UTF-8.

To create a variable or an array of this type, you must use `str` as the `type_of_value_allowed`.

#### 2.3.2. Number

This type represents double-precision floating-point numbers based on the IEEE 754-2008 standard.

To create a variable or an array of this type, you must use `num` as the `type_of_value_allowed`.

#### 2.3.3. Boolean

This type represents a classic boolean value that can only take two values: true or false.

To create a variable or an array of this type, you must use `bool` as the `type_of_value_allowed`.

#### 2.3.4. Any

This is not a type in itself. In fact, it is just a keyword that allows a variable or an array to take any possible value.

To create a variable or an array of this type, you must use `any` as the `type_of_value_allowed`.

### 2.4. Reserved Words

In Stapang, the following words are reserved:

#### 2.4.1. True

The word `true` is reserved for the boolean true value.

#### 2.4.2. False

The word `false` is reserved for the boolean false value.

#### 2.4.3. Null

The word `null` is reserved to prevent its use as an identifier.

### 2.5. Data Structures

#### 2.5.1. Variables

A variable is a reserved space in memory used to store a value.

#### 2.5.2. Arrays

An array is a collection of n reserved memory positions, each used to store a value.

## 3. Instructions

### 3.1. Stack and Data Management

#### 3.1.1. Push

This instruction allows you to put a value on top of the stack.

The syntax of this instruction is as follows:
* `push value;`
* `push name_of_variable;`
* `push name_of_array[position_of_the_array];`

#### 3.1.2. Pop

This instruction allows you to take the value at the top of the stack and put it inside a variable or an array position.

The syntax of this instruction is as follows:
* `pop name_of_variable;`
* `pop name_of_array[position_of_the_array];`

#### 3.1.3. Print

This instruction takes the value at the top of the stack and prints it on the screen.

The syntax of this instruction is as follows:
* `print;`

#### 3.1.4. Read

This instruction allows you to read a value from the keyboard, determining the type of value you want to read.

The syntax of this instruction is as follows:
* `read type_of_value_to_read;`

### 3.2. Variables and Arrays

#### 3.2.1. Var

This instruction allows you to create a variable.

The syntax of this instruction is as follows:
* `var name_of_variable:type_of_value_allowed;`

#### 3.2.2. Arr

This instruction allows you to create an array with an exact number of positions.

The syntax of this instruction is as follows:
* `arr name_of_array[number_of_positions]:type_of_value_allowed;`
### 3.3. Arithmetic Operators
#### 3.3.1. Add

This instruction takes the top two values from the stack and adds them together in the following way:
* `first_value_of_stack + second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `add;`

#### 3.3.2. Sub

This instruction takes the top two values from the stack and subtracts them in the following way:
* `first_value_of_stack - second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `sub;`

#### 3.3.3. Mult

This instruction takes the top two values from the stack and multiplies them in the following way:
* `first_value_of_stack * second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `mult;`

#### 3.3.4. Div

This instruction takes the top two values from the stack and divides them in the following way:
* `first_value_of_stack / second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `div;`

#### 3.3.5. Mod

This instruction takes the top two values from the stack and calculates their modulo in the following way:
* `first_value_of_stack % second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `mod;`
### 3.4. Comparison Operators
#### 3.4.1. Equal

This instruction takes the top two values from the stack and checks whether they are equal. Two values are considered equal only if they have the same type and the same value. For example, the number `5` and the string `"5"` are not considered equal.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `equal;`

#### 3.4.2. Bigger

This instruction takes the top two values from the stack and checks whether one value is bigger than the other in the following way:
* `first_value_of_stack > second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `bigger;`

#### 3.4.3. Lower

This instruction takes the top two values from the stack and checks whether one value is lower than the other in the following way:
* `first_value_of_stack < second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `lower;`

#### 3.4.4. BiggerEqual

This instruction takes the top two values from the stack and checks whether one value is bigger than or equal to the other in the following way:
* `first_value_of_stack >= second_value_of_stack`

In the case of strings, lexicographical comparison is used.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `biggerEqual;`

#### 3.4.5. LowerEqual

This instruction takes the top two values from the stack and checks whether one value is lower than or equal to the other in the following way:
* `first_value_of_stack <= second_value_of_stack`

In the case of strings, lexicographical comparison is used.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `lowerEqual;`

### 3.5. Logical Operators
#### 3.5.1. And

This instruction takes the top two values from the stack and checks whether both values are true.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `and;`

#### 3.5.2. Or

This instruction takes the top two values from the stack and checks whether at least one of the values is true.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `or;`

#### 3.5.3. Not

This instruction takes the value at the top of the stack and, if it is a boolean value, returns its complement.

The resulting boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `not;`

### 3.6. Control Flow

#### 3.6.1. If

This instruction takes the value at the top of the stack and, if it is a boolean true, executes the instructions inside the if block. There is also the option to define an alternative block that will be executed if the value is boolean false.

The syntax of this instruction is as follows:
```
if{
  instructions
}
#Optional
else{
  instructions
}
```

#### 3.6.2. While

This instruction takes the value at the top of the stack and, if it is a boolean true, executes the instructions inside the while block. When the block of instructions finishes, the condition is checked again.

The syntax of this instruction is as follows:
```
while{
  instructions
}
```

## 4. Errors

### 4.1. Syntax Errors

Syntax errors occur while the interpreter is analyzing the code, before execution, due to invalid syntax.

#### 4.1.1. Unterminated String Error

This error occurs when a string literal is not properly closed with the character `"`.

#### 4.1.2. Invalid Character Error

This error occurs when a character that is not allowed in the language syntax is detected. An example would be `$`.

#### 4.1.3. Invalid Identifier Error

This error occurs when an identifier used for a variable or an array contains an invalid character.

#### 4.1.4. Keyword Expected Error

This error occurs when the interpreter expects an instruction to begin with a keyword but does not find a valid one.

#### 4.1.5. Value Expected Error

This error occurs when an instruction requires a value, but no value is provided. This includes explicit values and references to variables or arrays.

#### 4.1.6. Semicolon Expected Error

This error occurs when an instruction is not terminated with a `;`.

#### 4.1.7. Opening Bracket Expected Error

This error occurs when an instruction, such as `arr`, expects an opening bracket `[` but does not find one.

#### 4.1.8. Closing Bracket Expected Error

This error occurs when an instruction expects a closing bracket `]` but does not find one.

#### 4.1.9. Number Expected Error

This error occurs when an instruction expects a numeric value but does not find one.

#### 4.1.10. Opening Brace Expected Error

This error occurs when an instruction expects an opening brace `{` but does not find one.

#### 4.1.11. Closing Brace Expected Error

This error occurs when an instruction expects a closing brace `}` but does not find one.

#### 4.1.12. Colon Expected Error

This error occurs when an instruction expects a colon `:` but does not find one.

#### 4.1.13. Type Expected Error

This error occurs when an instruction expects a data type but does not find one.

### 4.2 Runtime Errors
