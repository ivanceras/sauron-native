https://news.ycombinator.com/item?id=19746440

comment from phtrivier in HN: Cross-Platform GUI Toolkit Trainwreck (2016)

```
So many sub-debates hidden in this "cross-platform native GUI":

1) I want to write a GUI code once

2) I want to ship something that works on Windows / Linux / MacOS

2.1) I want to ship something that looks the same on Windows / Linux / MacOS

3) I want to ship a binary that works on Windows, a binary that works on Linux, a binary that works

3.1) I want to ship a small and efficient binary

4) I want to ship something that looks like a Windows app on Windows, a Linux app on Linux, a MacOS app on MacOS, etc...

(By definition, "I want to ship something that looks the same everywhere and looks like an app of my host" is meaningless, right ?)

The "holy grail" seems to be:

5) "I want to write a GUI code once that generates small efficient binaries that looks exactly like an app of the host OS, and if possible looks the same everywhere, and let me go back to writing my business logic rather than agonize over drawing a button."

It's seems from the debate that nothing obvious fulfills 4 and 5.

Then it comes to which requirement you're ready to drop.

If you're ready to drop requirement 2) , I suspect you're doing MacOS specific, go for it ;)

If you're ready to drop requirement 1) , I suspect your managers / salespersons disagree.

I suspect your manager / salespersons do not care about requirement 3.1), but it's debatable. Use Qt/Electron, and ship something.

I suspect your manager / salespersons do not care about requirement 4), and I suspect they're esthetics, which can not be defended.

I gave up waiting for someone to make 5, and don't have the resources / skill / time to do it myself. And maybe we should stop caring and watch the sky instead.

I hope someone is able to get to 1 + 2.1 + 3.1 someday.

I'll use that.
```
