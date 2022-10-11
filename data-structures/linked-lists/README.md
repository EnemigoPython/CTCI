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

## sum lists 1
Time: O(l log(l)) where l is the longer of the two linked lists
Space: O(l) where the most OVER the longest node * 2 to be stored is linear (1 extra carry digit in the buffer/ret list)
Optimisation: I was happy with this until I realised it didn't iterate the right way, so I just reversed the iterable when I push to the return list... This adds log(l) to the time complexity but it is amortised into the existing complexity.

## sum lists 2
Time: O(a + b + l) - one iteration of each linked list (a, b) and another O(l) to process each digit into a new linked list
Space: O(l) - in this case the stored values used to create the new linked list are O(1), but the return list itself is O(l)
Optimisation: if the zip technique from the previous method was used here, the time would be O(l).

## palindrome
Time: O(N log(N)) where one iteration is required to convert the list into a stack and a worst case O(N) best case O(log(N)) for double iteration of the items. This final iteration is truly O(N/2) in the worst case.
Space: O(N) to store the list to a stack
Optimisation: this would literally all be better if I had implemented a double linked list and just checked it from both ends. This would have been O(1) space and O(N) worse case time.

## intersection
Time: O(a + b) worst case where a and b equal the lengths of the two lists. Worst case is reached if there are no intersections. Best case is O(a) if we find an intersection straight away, or O(a + log(b)) if it is quick.
Space: O(a + b) worst case if we assume neither list is cyclical and there are no intersections. We store the value in every node.
Optimisation: it feels like there might be a clever way to check if a node has been visited without storing the pointer or reference. The best I could think of was an additional field in the node to check iteration (a boolean) but this would add space to ALL nodes by virtue of the struct being larger. Perhaps this would make the operation less expensive in a vacuum though.

## loop detection
Time: O(N) worst case if there is no loop
Space: O(N) worst case if there is no loop
Optimisation: it's difficult to imagine a better way to do this, as the hash set stores the history of links and a single iteration is used.