# Smallest Countdown Number
## Background
The Chicken McNugget Theorem states that with the numbers 6, 9, and 11, the lowest possible number of McNuggets that you can't get is 43. I was curious enough to wonder what the smallest possible number is with the possible countdown numbers if you could manipulate the numbers to get exactly what was needed to make the resulting value.

The algorithm works by using binary representations to view if a number is possible to use and using binary operations to test if different ways of representing the playfield are compatible to mix together.

A Dockerfile is available for those that want to test the code without installing Rust natively.

This algorithm produces the highest number of 16762 in two minutes when the print statement telling the user what number the simulation is on is turned off.

An example of how the highest number can be produced is with the six number 7, 4, 3, 75, 50, and 25.
1. 75 * 7 = 525
2. 525 + 50 + 3 = 578
3. 25 + 4 = 29
4. 29 * 578 = 16762
