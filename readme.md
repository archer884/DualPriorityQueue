DualPriorityQueue
=================

From: [Daily Programmer #201](http://www.reddit.com/r/dailyprogrammer/comments/2vkwgb/20150211_challenge_201_practical_exercise_get/)

This implementation of a priority queue in rust is based on two structs: the queue itself and a queue item. The queue item stores two priorities and the item itself, while the queue holds onto a vector filled with queue items and provides a nice place for me to hook up the functions necessary to make this work.

Functions exported by the DualPriorityQueue are: `enqueue()`, `dequeue_p1()`, and `dequeue_p2()`. Both public dequeue functions actually just call a private `dequeue()` function which takes as a parameter a second function whose role is to select the right priority. (That part was fun for me.)

I wanted to implement a `dequeue_by()` function which would accept some user-provided function and allow items to be dequeued in whatever arbitrary manner you might come up with, but that didn't happen. It's late and I'm sleepy. I guess I could just mark the existing `dequeue()` function as public, but that one has fairly limited functionality.

This was actually a lot of fun to implement, and it was easier than I really expected it to be. I guess I'm getting used to Rust. And, of course, the guys working on it have been chipping away at the rough edges pretty diligently since I started working with it... If you browse through my code real quick, you can really see the effects of that: none of it looks ridiculously arcane and most of it no longer resembles line noise.

Having finished my implementation, I can safely say now that it would make more sense to me to build a priority queue that allowed the user to store a queue item with any number of priorities and then pull them out using `dequeue_by()`. Having just the two options seems arbitrarily limited, given that we have already decided that just one wasn't enough. Exactly how that would be achieved, I'm not sure. Honestly, you can already do that the way this is currently written, but then it seemst to me there is little need for the queue item struct itself. Without that, how do you mark a struct as being enqueueable? Maybe you can have a trait that doesn't actually involve any functions... No idea. Question for another time. 

Update: `dequeue_by()` has totally been implemented--specifically by exporting the existing `dequeue()` function and changing its name. As I mentioned, the functionality is limited, but that doesn't mean it's worthless: you can do some weird things with your priority function if you want to. Like, for example, in my test code, odd values are given priority over the next even value. :P In theory, you could average the priorities if you wanted, or sum them, or ... Whatever. Just as long as the result has the right traits, the queue doesn't care.

Update: `DualPriorityQueueItem` no longer boxes its contents. If you want to store the data right there in the vector, that's perfectly fine with me. (Makes plenty of sense, actually.) I mostly just did it the other way to see if I could write it that way. Also added a test in here to make sure you can box your own items if you want. It's not like I didn't think it would work, but, still--nice to have the test.
