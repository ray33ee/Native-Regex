# Native Regex

A library of tools to allow the compiling of Regex engines into programming language source code.

# Why?

Regexes are incredibly useful, without them string handling is diffucult at best, and at worst error prone and frustrating.
They do however incur a large performance overhead, especially when compiling the regex and when matching. 
The regex compile state happens at runtime which can adversly affect program startup times. 
The Regex engines are also not as efficient as hard-coded solutions. 

This project aims to alleviate these issues by treating a Regex as source code itself, and compiling at compile time.
This is possible since the vast majority of Regexes are known at compile time.
This has the following benefits

- Bad regexes are spotted at compile time
- No runtime overhead for compiling Regexes
- Faster regex matching and performance

# Unicode

Native Regex does not support Unicode characters. For performance reasons it also does not check for them either. If a unicode character is found, this may result in undefined behaviour
