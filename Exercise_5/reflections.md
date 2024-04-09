Part 5: Reflecting

Condition variables, Java monitors, and Ada protected objects are quite similar in what they do (temporarily yield execution so some other task can unblock us).

Q1: But in what ways do these mechanisms differ?

A1: Good question. Answer to this is left as an exercise for sensor.



Bugs in this kind of low-level synchronization can be hard to spot.

Q2: Which solutions are you most confident are correct?
    Why, and what does this say about code quality?

A2: Condition variable and semaphore. Not just because it had tests to verify the implementation, but also because the logic of using a priority queue in
    this way was easier to visualize mentally, and thereby also easier to implement logic for. It also reminded me more of the ways I've implemented similar
    logic in C in past projects. 



We operated only with two priority levels here, but it makes sense for this "kind" of priority resource to support more priorities.

Q3: How would you extend these solutions to N priorities? Is it even possible to do this elegantly?
    What (if anything) does that say about code quality?

A3: I would use the condition variable solution, and make sure my algorithm for sorting the priority queue is correct. I think this would be an
    elegant solution. Using semaphores with N priority levels would be really messy, and the priority select in go would be absolutely horrendous.



In D's standard library, getValue for semaphores is not even exposed (probably because it is not portable – Windows semaphores don't have getValue, though you could hack it together with ReleaseSemaphore() and WaitForSingleObject()).

Q4: A leading question: Is using getValue ever appropriate?
    Explain your intuition: What is it that makes getValue so dubious?

A4: I quess it's not ever appropriate, but I don't understand why it can't be appropriate for another purpose such as checking how many active users a resource have...




Q5: Which one(s) of these different mechanisms do you prefer, both for this specific task and in general? (This is a matter of taste – there are no "right" answers here)

A5: I prefer the conditional variable because I think it scales the best for multiple priorities, and I found the logic here to be one of the easiest to implement. 