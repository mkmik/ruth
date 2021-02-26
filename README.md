# ruth
rust thread priority playground

bench2.csv contains latency measurments on a 6 core linux VM on my laptop.

The first coulumn (int) is the number of "interactive workload threads".

The second column (batch) is the number of "batch workload threads" each running with priority 10.

The remaining columns are latency percentiles of some dummy cpu intensive workload.

caveat emptor: this code compiles on mac too but only on linux `setpriority` changes the priority of
the calling thread and not of the whole process.