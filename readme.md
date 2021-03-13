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


# Limitations

For various reasons, some features of common regexes are not yet supported

## Unicode

Native Regex does not support Unicode characters. For performance reasons it also does not check for them either. 
Checking for unicode is the responsibility of the user.

## Backtracking

Perhaps the next biggest limitation is lack of backtracking. Take the regex 

```regexp
([0-9]*)([0-9])
```

This will find a match in the string "0472894739". This match will produce the three captures "0472894739", "047289473" and "9".
This regex will not match under the Native Regex function. This is because the first capture group greedily matches the entire string, and so the second group fails to match, resulting in the entire string failing.

This problem is circumvented by backtracking. This generally involves trying a less greedy match for the previous match.
If this fails we keep trying different permutations until it matches. If we keep back tracking and no permutations work, the match fails.

Backtracking is not yet implemented, but it may be in the future.

## Alternation

Alternation is not supported since it can be achieved with multiple regexes.

Perhaps a system for combining multiple regexes will be implemented (similar to `RegexSet` from the [regex](https://github.com/rust-lang/regex) crate)

## Named groups and backreferences

Named groups and backreferences are not yet supported, this may change in the future.

## Lookaround

Lookarounds are not yet supported

## Free-Spacing

Free-spacing is not yet supported since whitespace can be removed by the user.
