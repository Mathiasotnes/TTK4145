Exercise 1 - Theory questions
-----------------------------

### Concepts

What is the difference between *concurrency* and *parallelism*?

> Concurrency is that one processor switches quickly between different tasks to
simulate parallelism. Different routines gets time to run og the processor from a scheduler.
Parallelism is that multiple things run in true parallell which can be done on a multicore processor.


What is the difference between a *race condition* and a *data race*? 

> Race condition is a term describing behaviours that are dependent on the sequence or timing
of uncontrollable events, while data race is a special case of race condition where multiple threads
are trying to access the same memory location without proper synchronization. 
 

*Very* roughly - what does a *scheduler* do, and how does it do it?

> It decides which thread that should run and when. It does this by checking which threads are ready to
run, and choose one of them at random. It can also interrupt a thread if a thread runs for too long. This
is usually implemented as a safety to make sure every thread gets runtime. 


### Engineering

Why would we use multiple threads? What kinds of problems do threads solve?

> It makes it easier to implement multiple blocks of code that should run independently of eachother.
It solves the problem of complexity that comes from entangled code. It makes it so that one block can be
changed independently of other blocks.


Some languages support "fibers" (sometimes called "green threads") or "coroutines"? What are they, and why would we rather use them over threads?

> Fibers are a lightweight version of threads that aren't managed by the operating system. Context switching in normal threads are 
usually tideus, and fibers can make the context switching a lot quicker because everything is implemented in the application layer. 


Does creating concurrent programs make the programmer's life easier? Harder? Maybe both?
> For SUPER simple programs it may be harder, for example when blinking 1 LED or maybe 2. But after it's implemented
then it's super scalable because there's no more work to add more LED's when the concurrency is set up. And if the program is complex
it's practically impossible to do it without. It therefore in most cases make the programmer's life a lot easier.


What do you think is best - *shared variables* or *message passing*?
> I like shared variables and mutexes. I'm a weirdo. Used global variables with mutexes on the propulse rocket Bifrost, such a weirdo move?


