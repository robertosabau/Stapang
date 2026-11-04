# Stapang
## What is Stapang?
Stepang is an interpreted statically typed programming language where every action of has to pass thow a stack.
## Basic Syntax
There is two factors to take care about of the syntax of the programming language.

The first one is that every instruction has to finish with a `;`

The second is that this programming is lenguage is whitespace-insensitive, this means that you can do all the intruction in one line or one line for each part of the instruction

## Instructions
### Pop
This instruction allows you to take a the value of the top of the stack and putting it inside a variable or a position of an array.

The syntax of this instruction is the following ones:
* `pop name_of_variable;`
* `pop name_of_array[position_of_the_array];`
  
### Push
This instruction allows you to put a value on the top of the stack.
The syntax of thsi instruction is the following ones:
* `push value;`
* `push name_of_variable;`
* `push name_of_array[position_of_the_array];`

### Var
This instruction allows you to create a variable.
The syntax of this instruction is the following one:
* `var name_of_variable:type_of_value_allowed;`

### Arr
This instruction allows you to create an array with a number exact of positions.
The syntax of this instruction is the following one:
* `arr name_of_array[number_of_positions]:type_of_value_allowed;`

### Print
This instruction takes the value on the top of the stack and print it on the screen.
The syntax of this instructios is the following one:
* `print;`

### Read
This instruction allows you to read a value from the keyworld, determining the type you want to read.
The syntax of this instruction is the following one:
*`read type_of_value_to_read;`

### Add
This instruction takes the first two values of the stack and makes its addition by the following way:
* `first_value_of_stack + second_value_of_stack`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `add;`

### Sub
This instruction takes the first two values of the stack and makes its substraction by the following way:
* `first_value_of_stack - second_value_of_stack`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `sub;`

### Mult
This instruction takes the first two values of the stack and makes its mutliplication by the following way:
* `first_value_of_stack * second_value_of_stack`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `mult;`

### Div
This instruction takes the first two values of the stack and makes its divition by the following way:
* `first_value_of_stack / second_value_of_stack`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `div;`

### Mod
This instruction takes the first two values of the stack and makes its module by the following way:
* `first_value_of_stack & second_value_of_stack`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `mod;`

### Equal
This instruction takes the first two values of the stack and checks if there are the same value, checking if the values are the same type and value. In other way `5!="5".`

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `equal;`

### Bigger
This instruction takes the first two values of the stack and checks if one value is bigger than the other by the following way:
* `first_value_of_stack > second_value_of_stack`

The resultant value is putted in the top of the stack.

In case of the strs lexicographic comparison is used

The syntax of this instruction is the following one:
* `bigger;`

### Lower
This instruction takes the first two values of the stack and checks if one value is lower than the other by the following way:
* `first_value_of_stack < second_value_of_stack`

The resultant value is putted in the top of the stack.

In case of the strs lexicographic comparison is used

The syntax of this instruction is the following one:
* `lower;`
### BiggerEqual
This instruction takes the first two values of the stack and checks if one value is bigger or equal than the other by the following way:
* `first_value_of_stack >= second_value_of_stack`

The resultant value is putted in the top of the stack.

In case of the strs lexicographic comparison is used

The syntax of this instruction is the following one:
* `biggerEqual;`

### LowerEqual
This instruction takes the first two values of the stack and checks if one value is lower or equal than the other by the following way:
* `first_value_of_stack < second_value_of_stack`

The resultant value is putted in the top of the stack.

In case of the strs lexicographic comparison is used

The syntax of this instruction is the following one:
* `lowerEqual;`

### And
This instruction takes the first two values of the stack and checks if the two values are true.

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `and;`

### And
This instruction takes the first two values of the stack and checks if one of the values are true.

The resultant value is putted in the top of the stack.

The syntax of this instruction is the following one:
* `or;`

### Not
This instruction takes the first value of the stack and if it is a boolean value makes its complementary

The resultant value is putted in the top of the stack

The syntax of this instruction is the following one:
* `not;`

### Safe
This instruction allows to change the value of the safe mode.

The syntax of this instruction is the following one:
*`safe boolean_value;`
#### What is the safe mode
The save mode is a runtime configuration that can be change during the execution of the programm and allows to the programmer to determined if whants to have or the errors or ignore some errors that would finish the execution. 

By default this mode is on, which means that all the errors are given. In case is set false with the safe instruction will desactive the trow of some terminal errors.

Any presence of a safe instruction will release a warning before starting the execution.

When the safe mode is off, the errors are replaced by values like Null or NaN.

### If
This instruction takes the first value of the stack and if it is a boolean true it will execute the instructions inside the if. There is the option to set an alternative way if it reads a boolean false

The syntax of this instructios is the following one:
```
if{
  instructions
}
#Optional
else{
  instructions
}
```

### While
This instruction takes the first value of the stack and if it is a boolean true it will execute the instructions inside the if. When it finishes the block of instruction checks again 

The syntax of this instructios is the following one:
```
while{
  instructions
}
```

## Types
In Stapang exists the following types for variables and arrays:
### Strings
This type represents character strings UTF-8 encoder.

To create a variable or an array of this type has to put as type_of_value_allowed `str`.

### Number
This type represents double-precision floating-point numbers based on the IEEE 754-2008 standard.

To create a variable or an array of this type has to put as type_of_value_allowed `num`.

### Boolean
This type representans a classic boolean value that can only take two values, true o false.

To create a variable or an array of this type has to put as type_of_value_allowed `bool`.

### Any
This is not a type by its self, in fact its just a keyworld that allows to a variable or array to take any posible value

To create a variable or an array of this type has to put as type_of_value_allowed `any`.

## Comments
It only exists comments in the same line and to make a comment you have to use the caracter `#`, which will make the interpreter to ignore all behind the `#` until the first `\n`.

## Reservated words
In Stepang the following worlds are reserved:
### True
The word `true` is reserved for the boolean true value.
### False
The word `false` is reserved for the boolean false value.
### Null
The word `null` is reserved to avoid its use.
