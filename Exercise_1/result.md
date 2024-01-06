# 3: Sharing a variable

When using runtime.GOMAXPROCS(2), I get the magic number to be -614 184. This it likelly because both routines are using the shared
variable at the same time which creates a race condition where they are trying to manipulate the piece of memory at the same time.
The results ends up being a random number between -1 000 000 and 1 000 000.

When using runtime.GOMAXPROCS(1), I get the magic number to be 0. This is because the first routine runs until it's finished before the
next routine starts. This makes so that the routines don't interrupt eachothers operations, but it makes the routines practically blocking
because only one routine can run at a time. 
