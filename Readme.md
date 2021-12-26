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

- [**Day 07**](https://adventofcode.com/2021/day/7)

  Ugh the rounding issues on the second subtask are so annoying.

- [**Day 08**](https://adventofcode.com/2021/day/8)

  Friendship ended with [nom][crate_nom]. Now [combine][crate_combine] is my
  best friend.

- [**Day 09**](https://adventofcode.com/2021/day/9)

  Knowing the properties revealed in the second subtask makes the first subtask
  simpler. We don't have to check for local minima anymore, instead we just
  look for the minimum within a basin.

  Both subtasks can now be solved with some kind of union-find where we keep
  the size and minimum of each set.

- [**Day 10**](https://adventofcode.com/2021/day/10)

  Nice, just pass in the remaining stack to the scoring function.

- [**Day 11**](https://adventofcode.com/2021/day/11)

  Weird, simply iterating over the whole field multiple times until no more
  changes occur is not that much slower than having a queue system of changes.

- [**Day 12**](https://adventofcode.com/2021/day/12)

  Noooooo this task is demolishing the performance of my test suite, it's
  taking almost half a second in debug and 13ms in release :(:(

- [**Day 13**](https://adventofcode.com/2021/day/13)

  Nope I'm not actually recognizing the characters here, instead there's a
  switch in the code to print out the resulting pattern after folding the
  paper. This means that only the first subtask is tested, I hope there's not
  gonna be any more tasks of this type.

- [**Day 14**](https://adventofcode.com/2021/day/14)

  You'll notice that I have completely given up on using parser libraries, it's
  just not worth it in these simple cases.

- [**Day 15**](https://adventofcode.com/2021/day/15)

  A* improves the speed by almost a factor 2 over plain Dijkstra.

  I also tried both filling in the full 25x map as well as computing the value
  of a cell whenever I visit it and only storing the original map. Using more
  memory is faster.

- [**Day 16**](https://adventofcode.com/2021/day/16)

  [deku][crate_deku] is such a good library!

- [**Day 17**](https://adventofcode.com/2021/day/17)

  The version where I only iterate over the timesteps instead of over the x
  velocities times the amount of hits for this x velocity is slightly faster
  but the code there is hideous and I'm not completely sure if my solution
  there properly generalizes to other inputs.

- [**Day 18**](https://adventofcode.com/2021/day/18)

  Rust's macro_rules! is lovely :) We can construct snail numbers like this:
  ```rust
  number![[[5,[7,4]],7],1]
  ```

  As for the algorithm, I can't think of anything smarter to do in the second
  subtask than to just iterate over all pairs, but it's just 100 entries so
  who cares.

- [**Day 20**](https://adventofcode.com/2021/day/20)

  Look out for the boundary when zero maps to on and 511 maps to off, because
  in that case the entire empty space beyond the frame should be toggling on
  and off (this is why only even numbers of iterations are used in the task,
  otherwise the count of active cells would be infinite).

  When clipping the grid to a finite region, this toggling might lead to
  artifacts at the boundary.

- [**Day 23**](https://adventofcode.com/2021/day/23)

  Another A* task. I first missed the constraint where no amphipod can move
  into someone else's room, which had the effect of blowing up the release
  runtime from 0.08s up to "exceeding my patience".

## [2020](https://adventofcode.com/2020)

- [**Day 01**](https://adventofcode.com/2020/day/1)

  For the first subtask using a hashset shouldn't be much faster than just 
  sorting the array because the input is so small, but using the hashmap for
  the second subtask seems like a much more natural representation.

- [**Day 02**](https://adventofcode.com/2020/day/2)

  Some more [nom][crate_nom] practice. The most difficult thing about this at
  the moment is getting type deduction to work without a ton of boilerplate.

- [**Day 03**](https://adventofcode.com/2020/day/3)

  Another constant-ish memory solution (at least with respect to the height of
  the input).

- [**Day 25**](https://adventofcode.com/2020/day/25)

  Huh am I missing something? For this to be a task here I'd expect there to be
  a quicker way to get this logarithm than just looping, otherwise the task is
  not even interessting??

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
[crate_nom]: https://crates.io/crates/nom
[crate_combine]: https://crates.io/crates/combine
[crate_deku]: https://crates.io/crates/deku
