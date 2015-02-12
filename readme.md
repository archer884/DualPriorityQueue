DualPriorityQueue
=================

From: [Daily Programmer #201](http://www.reddit.com/r/dailyprogrammer/comments/2vkwgb/20150211_challenge_201_practical_exercise_get/)

This implementation of a priority queue in rust is based on two structs: the queue itself and a queue item. The queue item stores two priorities and a reference to the boxed item, while the queue holds onto a vector filled with queue items and provides a nice place for me to hook up the functions necessary to make this work.

Functions exported by the DualPriorityQueue are: `enqueue()`, `dequeue_p1()`, and `dequeue_p2()`. Both public dequeue functions actually just call a private `dequeue()` function which takes as a parameter a second function whose role is to select the right priority. (That part was fun for me.)

I wanted to implement a `dequeue_by()` function which would accept some user-provided function and allow items to be dequeued in whatever arbitrary manner you might come up with, but that didn't happen. It's late and I'm sleepy. I guess I could just mark the existing `dequeue()` function as public, but that one has fairly limited functionality.

This was actually a lot of fun to implement, and it was easier than I really expected it to be. I guess I'm getting used to Rust. And, of course, the guys working on it have been chipping away at the rough edges pretty diligently since I started working with it... If you browse through my code real quick, you can really see the effects of that: none of it looks ridiculously arcane and most of it no longer resembles line noise.
