# Multi-Level Feedback Queue (MLFQ) Scheduler

Simple MLFQ Scheduler in Rust for CS5600

```zsh
$ cargo run -- -q 10,20,30,40,50 -a 20,40,60,80,100 -j 20,100,0,0:30,100,50,1:35,50,5,5 -b 100 -i
[20:<S>] CPU idle for 20 ticks.
[20:<0>] Process 0 start running.
[30:<0>] Process 0 has run for 10.
[30:<0>] Process 1 start running.
[40:<0>] Process 1 has run for 10.
[40:<0>] Process 2 start running.
[45:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[45:<0>] Process 2 bumped after I/O.
[55:<0>] Process 0 has run for 10.
[55:<S>] Process 0 priority reduced to 1.
[65:<0>] Process 1 has run for 10.
[65:<S>] Process 1 priority reduced to 1.
[65:<0>] Process 2 resume running from I/O.
[70:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[70:<0>] Process 2 bumped after I/O.
[70:<0>] Process 2 resume running from I/O.
[75:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[75:<0>] Process 2 bumped after I/O.
[75:<0>] Process 2 resume running from I/O.
[80:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[80:<S>] Process 2 priority reduced to 1.
[80:<1>] Process 2 resume running from I/O.
[85:<1>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[85:<1>] Process 2 bumped after I/O.
[105:<1>] Process 1 has run for 20.
[105:<S>] Priority boosted for all processes.
[115:<0>] Process 1 has run for 10, then blocked. It will perform I/O for 1
[115:<0>] Process 1 bumped after I/O.
[115:<0>] Process 2 resume running from I/O.
[120:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[120:<0>] Process 2 bumped after I/O.
[130:<0>] Process 0 has run for 10.
[130:<0>] Process 1 resume running from I/O.
[140:<0>] Process 1 has run for 10.
[140:<S>] Process 1 priority reduced to 1.
[140:<0>] Process 2 resume running from I/O.
[145:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[145:<0>] Process 2 bumped after I/O.
[155:<0>] Process 0 has run for 10.
[155:<S>] Process 0 priority reduced to 1.
[155:<0>] Process 2 resume running from I/O.
[160:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[160:<0>] Process 2 bumped after I/O.
[160:<0>] Process 2 resume running from I/O.
[165:<0>] Process 2 has run for 5, then blocked. It will perform I/O for 5
[165:<S>] Process 2 priority reduced to 1.
[165:<1>] Process 2 resume running from I/O.
[170:<1>] Process 2 has run for 5, then finished.
[170:<S>] Process 2 finished. Response time: 5. Turnaround time: 135.
[190:<1>] Process 0 has run for 20.
[210:<1>] Process 1 has run for 20.
[210:<S>] Priority boosted for all processes.
[220:<0>] Process 1 has run for 10.
[230:<0>] Process 0 has run for 10.
[240:<0>] Process 1 has run for 10, then finished.
[240:<S>] Process 1 finished. Response time: 25. Turnaround time: 210.
[250:<0>] Process 0 has run for 10.
[250:<S>] Process 0 priority reduced to 1.
[270:<1>] Process 0 has run for 20, then finished.
[270:<S>] Process 0 finished. Response time: 25. Turnaround time: 250.
All processes finished.
Total idle time: 20.
Average turnaround time: 198.
Average response time: 18.
```
