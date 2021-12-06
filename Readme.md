# Advent of Code in Rust

Spoilers ahead on how (not) to do it.

I don't claim that these solutions are the most elegant ones, or that
they work for all possible inputs, but they do work for mine.

## [2021](https://adventofcode.com/2021)

- [**Day 01**](https://adventofcode.com/2021/day/1)

  Nothing really interesting, but always nice to see problems that can be
  solved in constant memory.

  I guess technically this implementation is not constant memory if you
  don't consider the window size to be a constant, but if the window size
  ever gets really big you could just open the input file twice.

- [**Day 02**](https://adventofcode.com/2021/day/2)

  Again a job for [try_fold()][doc_try_fold]. Like on day 1 we can run both
  subtasks in the same run, we're thus only iterating through the data once.

- [**Day 03**](https://adventofcode.com/2021/day/3)

  Lol nested binary search, don't think I've seen that one before :D

  The runtime for the second subtask is O(b * n * log n) where n is the number
  of lines in the input and b is the number of bits per line.

- [**Day 04**](https://adventofcode.com/2021/day/4)

  Let `n` be the amount of bingo numbers drawn, `b` the amount of boards, and
  `s` the side length of a board. Then:

  * Runtime: O(b * n)
  * Storage: O(b * s * s + n)

- [**Day 05**](https://adventofcode.com/2021/day/5)

  By far the slowest code in this repo so far, will have to revisit this one
  later. Right now I'm just following all lines and entering the visited
  positions in a `HashMap`, this takes about 10ms in release mode.

  Update: lol just going from `HashMap` to a dense array of `u8`s improves the
  speed by a factor of 10. I guess a dense 1MB array sometimes really is the 
  best option.

- [**Day 06**](https://adventofcode.com/2021/day/6)

  DP. If you squint hard enough then it kinda looks like Fibonacci so I might
  also try the log solution at some later point.

## [2020](https://adventofcode.com/2020)

- [**Day 01**](https://adventofcode.com/2020/day/1)

  For the first subtask using a hashset shouldn't be much faster than just 
  sorting the array because the input is so small, but using the hashmap for
  the second subtask seems like a much more natural representation.

- [**Day 02**](https://adventofcode.com/2020/day/2)

  Some more [nom][doc_nom] practice. The most difficult thing about this at the
  moment is getting type deduction to work without a ton of boilerplate.

- [**Day 03**](https://adventofcode.com/2020/day/3)

  Another constant-ish memory solution (at least with respect to the height of
  the input).

## [2019](https://adventofcode.com/2019)

- [**Day 01**](https://adventofcode.com/2019/day/1)

  Just crunch the numbers, duh

- [**Day 02**](https://adventofcode.com/2019/day/2)

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
  * Evaluate polynomial, then bruteforce: O(n^2)
  * Diophantine equation: O(n)
  
- [**Day 04**](https://adventofcode.com/2019/day/4)

  Reading the first subtask I first thought I'd have to find some cool trick to
  iterate over numbers with nondecreasing digits with pairs in them, but taking
  the second subtask into account as well it seems easier to just iterate over
  numbers with nondecreasing digits and then filter out the rest.

  In the case of my inputs, going from iterating over all passwords between the
  lower and upper bounds down to iterating just over the nondecreasing ones
  reduces the required checks by a factor of over 200.

[doc_try_fold]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold
[doc_nom]: https://crates.io/crates/nom