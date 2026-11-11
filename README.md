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

### 2.1. Basic Syntax

There are two important rules to consider regarding the syntax of Stapang.

The first rule is that every instruction must end with a `;`.

The second rule is that Stapang is whitespace-insensitive. This means that you can write all the instructions on one line or use one line for each part of the instruction.

### 2.2. Comments

Stapang supports single-line comments. To write one, use the `#` character. The interpreter ignores everything after it until the next newline (`\n`).

### 2.3. Data Types

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

#### Index

This type represents an unsigned integer whose size depends on the target architecture.

To create a variable or an array of this type, you must use `index` as the `type_of_value_allowed`.

This is the only kind of value that can be used as index in arrays

#### Boolean

This type represents a Boolean value that can only have one of two possible values: `true` or `false`.

To create a variable or an array of this type, you must use `bool` as the `type_of_value_allowed`.

#### Any

`any` is not a data type itself. Instead, it is a keyword that allows a variable or an array to store values of any supported data type.

To create a variable or an array of this type, you must use `any` as the `type_of_value_allowed`.

### 2.4. Reserved Words

In Stapang, the following words are reserved:

#### True

The word `true` is reserved to represent the Boolean value `true`.

#### False

The word `false` is reserved to represent the Boolean value `false`.

#### Null

The word `null` is reserved to prevent its use as an identifier.

### 2.5. Data Structures

#### Variables

A variable is a named storage location used to hold a value.

When a variable is created, it is not initialized.

Variables are mutable.

#### Arrays

An array is an indexed collection of positions, each capable of holding a value.

Array indexing starts at `0`.

When an array is created, all its positions are uninitialized.

Arrays are mutable.

## 3. Instructions

### 3.1. Stack and Data Management

#### Push

This instruction allows you to put a value on top of the stack.

The syntax of this instruction is as follows:
* `push value;`
* `push name_of_variable;`
* `push name_of_array[position_of_the_array];`

#### Pop

This instruction allows you to take the value at the top of the stack and put it inside a variable or an array position.

The syntax of this instruction is as follows:
* `pop name_of_variable;`
* `pop name_of_array[position_of_the_array];`

#### Print

This instruction takes the value at the top of the stack and prints it to the standard output.

The syntax of this instruction is as follows:
* `print;`

#### Read

This instruction allows you to read a value from the standard input and specify its expected data type.

The read instruction currently supports `str`, `int`, `float` and `index` as input types.

The syntax of this instruction is as follows:
* `read type_of_value_to_read;`

### 3.2. Variables and Arrays

#### Var

This instruction allows you to create a variable.

The syntax of this instruction is as follows:
* `var name_of_variable:type_of_value_allowed;`

#### Arr

This instruction allows you to create an array with a specified number of positions.

The syntax of this instruction is as follows:
* `arr name_of_array[number_of_positions]:type_of_value_allowed;`

### 3.3. Arithmetic Operators

Since all values are handled through the stack, which follows the LIFO (Last In, First Out) principle, `first_value_of_stack` refers to the value at the top of the stack when the instruction is executed. `second_value_of_stack` refers to the value immediately below it, which was pushed onto the stack before the first value.

#### Add

This instruction adds the top two values on the stack.
* `first_value_of_stack + second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `add;`

#### Sub

This instruction subtracts the top two values on the stack.
* `first_value_of_stack - second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `sub;`

#### Mult

This instruction multiplies the top two values on the stack.
* `first_value_of_stack * second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `mult;`

#### Div

This instruction divides the top two values on the stack.
* `first_value_of_stack / second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `div;`

#### Mod

This instruction calculates the remainder of the division of the top two values on the stack.
* `first_value_of_stack % second_value_of_stack`

The resulting value is put on top of the stack.

The syntax of this instruction is as follows:
* `mod;`

### 3.4. Comparison Operators

Since all values are handled through the stack, which follows the LIFO (Last In, First Out) principle, `first_value_of_stack` refers to the value at the top of the stack when the instruction is executed. `second_value_of_stack` refers to the value immediately below it, which was pushed onto the stack before the first value.

#### Equal

This instruction takes the top two values from the stack and checks whether they are equal. Two values are considered equal only if they have the same type and the same value. For example, the number `5` and the string `"5"` are not considered equal.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `equal;`

#### Greater

This instruction takes the top two values from the stack and checks whether the first value is greater than the second.
* `first_value_of_stack > second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `greater;`

#### Less

This instruction takes the top two values from the stack and checks whether the first value is less than the second.
* `first_value_of_stack < second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `less;`

#### GreaterEqual

This instruction compares the top two values on the stack and checks whether the first value is greater than or equal to the second.
* `first_value_of_stack >= second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `greaterEqual;`

#### LessEqual

This instruction compares the top two values on the stack and checks whether the first value is less than or equal to the second.
* `first_value_of_stack <= second_value_of_stack`

In the case of strings, lexicographical comparison based on UTF-8 bytes (case-sensitive) is used.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `lessEqual;`

### 3.5. Logical Operators

#### And

This instruction takes the top two values from the stack and checks whether both values are the Boolean value `true`.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `and;`

#### Or

This instruction takes the top two values from the stack and checks whether at least one of the values is the Boolean value `true`.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `or;`

#### Not

This instruction takes the value at the top of the stack and, if it is a Boolean value, returns its logical negation.

The resulting Boolean value is put on top of the stack.

The syntax of this instruction is as follows:
* `not;`

### 3.6. Control Flow

#### If

This instruction takes the value at the top of the stack. If the value is the Boolean value `true`, the instructions inside the if block are executed. Otherwise, an optional else block can be executed if the value is the Boolean value `false`.

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

#### While

This instruction takes the value at the top of the stack. If it is the Boolean value `true`, the instructions inside the while block are executed. Once the block finishes executing, the interpreter takes the value at the top of the stack again and checks whether it is `true`. This process continues as long as the condition evaluates to `true`.

The syntax of this instruction is as follows:
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

#### Cannot Convert to Number Error

This error occurs when an attempt is made to convert an invalid string into a number. This usually happens when the read instruction receives input that cannot be converted to the requested numeric type.

#### Invalid Value Error

This error occurs when an attempt is made to assign a value to a variable or an array position that is incompatible with its declared type. For example, assigning a Boolean value to a variable declared as `str`.

#### Variable Not Initialized Error

This error occurs when an attempt is made to retrieve the value of a variable that has not been initialized.

#### Array Position Not Initialized Error

This error occurs when an attempt is made to retrieve the value of an array position that has not been initialized.
