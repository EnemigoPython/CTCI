# arrays and strings
The first chapter in the book, with lots of questions about manipulating these structures through simple iteration. No question was particularly challenging but rotate matrix was the most fun.

## is unique
- Time: O(N) for 1, O(N^2) for 2
- Space: O(log(N)) best case, O(N) worst for 1, O(1) for 2
- Optimisation: 
As the prompt indicates, the trade-off can be made to sacrifice time for no extra space overhead.

## check permutation
- Time: O(AB) -- each string is traversed one time respectively
- Space: O(log(A)) average, O(A) worst -- if there are no repeating letters the HashMap will contain every letter of string A as a key
- Optimisation:
If the string was sorted each letter could be compared in a single loop and space would be O(1). This would add O(N log(N)) time however.

## URLify
- Time: O(N)
- Space: O(N)
- Optimisation:
I did not do this in place - if I had there would be no need for a string copy. Doing the operation in place would require pushing characters forward in the string, as in a char array, for each replaced space. This would add O(log(N)) * S to time where S is the number of spaces in an input string.

## palindrome permutation
- Time: O(N)
- Space: the same as for *check permutation*, where in cases where there are at least some repetitions of characters it will be O(log(N))
- Optimisation:
One iteration is probably the most we can expect in terms of time. Again, a way to make space O(1) would be to sort it in place, with the discussed trade-offs.

## one away
- Time: O(a) where a is the shortest length of either of the two strings
- Space: O(1)
- Optimisation:
There is only one iteration comparing the strings and space is constant regardless of string size. This feels optimal.

## string compression
- Time: O(N)
- Space: average case O(N), best case O(log(N))
- Optimisation:
Because the length of the "compression" relative to the original string is checked while building, space is guaranteed to be only O(N). If this check wasn't made, the result could be O(N log(N)) when the compressed version grows bigger. 

## rotate matrix
- Time: O(N) * O(log(N)) -- the outer loop is N / 2 and the inner is N-i which expands to N
- Space: O(1) -- the matrix is mutated in place
- Optimisation:
I believe this is the fewest steps required to rotate all "pixels".

## zero matrix
- Time: O(MN) + Z*N where Z is the number of 0s in an array
- Space: O(Z), or O(log(MN)) -- the coords of 0 are stored in sets
- Optimisation:
One set could have been used instead of two to store a tuple. This would have saved a layer of iterating over found 0s for row AND column values. Another approach is that the matrix could have been copied and the copy mutated during the iteration. This would make the actual operation O(MN) - however it would incur the cost of the copy in both time and space.

## string rotation
- Time: O(N) + `is_substring` complexity (O(N)) = O(N)
- Space: O(N) -- after the O(1) constant work of finding the length of the second half substring, an index is stored of the front-end substring. This is worst case O(N) if it is essentially the entire string. On average doubling the size of the string doubles the range of lengths this index could have.
- Optimisation:
If we were to ignore `is_substring` altogether, we could iterate the front of the string to check if it corresponds to the remainder of the other string. Using just the index and iterating one character at a time would mean no chunk is stored in memory and take space complexity to O(1). This would however also take another O(log(N)) or worst case O(N) to traverse the string again.