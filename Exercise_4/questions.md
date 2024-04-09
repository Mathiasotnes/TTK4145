q1:
Why would we aim to detect success in results, instead of the more classical way of detecting errors/failures?

a:
It's impossible to detect all possible faults, because the number of faults are "infinite", so it's easier to 
detect when it's working. It's more secure, but maybe redundant. 

q2:
Why would we want to self-terminate, instead of handling the error immediately in the "primary" (as opposed to deferring it to the backup as it "spins up")?

a:
It's easier with a one-fits-all solution for handling errors, instead of writing code for handling all possible
errors.


q3:
Is there any reason to prefer a process pair style, as opposed to making a separate supervisor-like program whose sole purpose is to restart the main program?

a:
If there is a problem with the main program, it may not get resolved by a reset.
If the restart process is comprehensive, it may be a bad solution to restart.

