# linked lists
This one was a journey and a half, mostly because I decided to implement the Linked List manually as opposed to using the in-built `std::collections::LinkedList`. This decision was so that I could learn more about recursive structures in Rust. It wasn't as easy as I thought because of the ownership complications that Rust introduces, where I think it would be much more straightforward in C or C++.

## remove dups
Time: O(N)
Space: O(log(N)) - we only store a set of unique values
Optimisation: to do this in O(1) space we could remove duplicates of every value in the list in turn. This would make it worst case O(N^2) time complexity.

## kth
Time: O(N + k) where k is the index value
Space: O(N) when we convert the list to a vector
Optimisation: this is not an optimal implementation - rather than make the conversion a lazy iteration of the list would be O(k) time and O(1) space. It was done this way more for ease of syntax on my part.

## delete middle node
Time: O(N) worst case
Space: O(1)
Optimisation: nothing additional needs to be stored and just a single iteration of the list is required. This is optimal.

## partition
Time: O(N) - each node visited once
Space: O(N) average without assumptions about how many values will be moved to the buffer
Optimisation: this is done in place - to reduce space to O(1) values could be lifted out one by one and pushed to the front of the list, but then housekeeping of how many values have already been partitioned would be necessary and it would be a slow iteration of O(N^2).