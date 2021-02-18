# Advent of Code in Rust

Starting with 2019.

Please don't take any of this as serious advice on how to write idiomatic rust.

## Solutions 2019

I don't claim that these solutions are the most elegant ones, or that
they work for all possible inputs, but they do work for mine.

- *Day 01*

  Just crunch the numbers, duh

- *Day 02*

  Note that only in the first instruction we read from / write to an address
  that depends on the noun and verb. The result of this operation is overwritten
  in the second instruction. The end result thus doesn't depend on a memory
  access where the address is a function of the noun and verb.

  Thanks to this insight we can consider the output to be a series of additions
  and multiplications of only constants and the noun and verb, which we'll call
  x and y respectively. Running the program is thus equivalent to evaluating
  a bivariate polynomial.

  I further noticed that the resulting expression is linear in both x and y,
  without any cross terms. Subtask 2 can therefore be restated as finding the
  solution to a linear diophantine equation in two variables: ax + by = c. In
  non-degenerate cases, this equation has infinitely many solutions for x and
  y, but the problem statement was nice enough to further constrain the two
  variables. Both x and y are used as memory addresses in the first instruction,
  which requires them both to be non-negative and less than the length of the
  given Intcode program. This narrows it down to a single solution.

  You could of course also just brute force the solution, there are only n^2
  possible inputs where n is the length of your Intcode program, so this should
  take less than a second.

  Here are the asymptotic runtimes, assuming that the Intcode program has
  the property that it evaluates to a diophantine equation.

  * Bruteforce: O(n^3)
  * Evaluate Polynomial, then bruteforce: O(n^2)
  * Diophantine equation: O(n)
