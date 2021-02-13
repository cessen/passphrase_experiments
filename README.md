# Passphrase Generation Experiments

This repository is an experiment in letting users create more memorable passphrases.  It's in the same spirit as diceware, but with more flexibility.  There are two main aspects to this:

1. Providing a word list that only has one form of each word.  For example, it only contains "cat", not "cats".  And only "eat", not "ate", "eats", or "eating".  And only "quick", not "quickly" or "quickness".
2. Giving the user the option to rearrange the words of their passphrase however they like, and providing accurate lower bounds on entropy when they do so.

Together, these two points give the user a lot of flexibility in rearranging and changing their passphrase into something memorable.  All while giving accurate lower bounds on entropy.


## Some examples.

To generate a 6-word passphrase, we can call the software like so:

```
$ ./passphrase -l 6
hand poet truce exist health sauce

Entropy in this order:  72 bits
Entropy rearranged:     62 bits
```

We can use these words as-is if we like, in which case we get 72 bits of entropy.  But alternatively we can change them around like this:

```
Healthy saucy poet hands exist.  Truce.
```

The capitalization and punctuation is superfluous.  Thanks to the design of the source word list, the change of word forms is also superfluous, and doesn't affect the entropy.  (Importantly, that isn't true of most diceware word lists.)

On the other hand, by rearranging the words, we've lost 10 bits of secure entropy.  Losing 10 bits of entropy is nothing to sneeze at, but what we get in exchange is a goofy and memorable passphrase.  And this becomes even more powerful with longer passphrases, which are increasingly difficult to memorize without some kind of context.  For example:

```
$ ./passphrase -l 10
coma trauma blank erode cut goofy spa flee critic brag 

Entropy in this order:  120 bits
Entropy rearranged:     98 bits
```

```
Coma critic flees eroding spa. Goofy's trauma blankly cuts. Brag. 
```

This time we've lost 22 bits of entropy.  But the original passphrase, in the order given, would be impractical for most people to memorize.  The rearranged passphrase, on the other hand, is probably quite doable.  *Especially* when the user is the one that creatively rearranges the words, forming a silly story in their head while doing so.


## How to use this software.

For secure passphrase generation, I recommend *not* using this software directly.  Rather, you should use it to generate a diceware list appropriate to the dice you have available, and then take the standard diceware approach using the generated list.

You can output a list appropriate for d8 dice like so:

```
$ ./passphrase -d 8
1-1-1-1    abandon
1-1-1-2    abduct
1-1-1-3    ability
...
```

By default, the list has 4096 entries, which perfectly matches four d8 dice.

If you have the more common d6 dice, you can generate a list for them like this:

```
$ ./passphrase -d 6
1-1-1-1-1    abandon
1-1-1-1-2    abduct
1-1-1-1-3    ability
...
```

You can also limit the length of the words allowed in the word list.  For example, if you want a maximum of 5-letter words:

```
$ ./passphrase -m 5 -d 6
1-1-1-1-1    abort
1-1-1-1-2    about
1-1-1-1-3    above
...
```

Note that by using these options you can end up with a number of words that won't match the dice you have.  In fact, the only case that will match exactly is using d8 dice with the full word list.

In case of such a mismatch, follow the normal diceware procedure anyway, but for each word keep re-rolling until you get a sequence that's on the list.  This will take extra time, and you may need to re-roll quite a few times for each word depending on the extent of the mismatch.  But doing things this way should keep your word selection unbiased.


## About the word list.

The default word list used by the software is `word_list.txt` in the root of the repo.

It is very much a work-in-progress.  The goal is to avoid multiple forms of the same word, but at the moment there are almost certainly quite a few slip ups.

Even with its current flaws, I think the list is pretty good, and should be safe to use as outlined in this readme without significant loss of entropy.  However, I do want to improve it further over time.  If you would like to help out with that, even just a cursory glance at some random part of `word_list.txt` would help out.

In addition to the security-oriented goals, I also have the following further usability goals for the list.  Words in the list should:

* Be easy to spell.
* Be widely known and understood (though not necessarily commonly used).
* Have meanings and nuances that lend themselves to memorable mental stories.

As far as these additional criteria go, up to this point I've just been selecting words based on how I feel about them and how I *think* other people would feel about them.  But that's obviously subject to my own biases.  Feedback on the list from this perspective is also welcome.


## Re-ordered entropy.

When using the full 4096 word list, the following is the lower bound on entropy when reordering the words is allowed:

* 2 words: 23 bits
* 3 words: 33 bits
* 4 words: 43 bits
* 5 words: 53 bits
* 6 words: 62 bits
* 7 words: 71 bits
* 8 words: 80 bits
* 9 words: 89 bits
* 10 words: 98 bits
* ...
* 15 words: 139 bits

Unlike with ordered passphrases, the bits of entropy per word decreases with the length of the passphrase.  But for reasonable passphrase lengths, a good rule of thumb is about 10 bits per word.


## Future work.

Other ideas I would like to explore in the future are:

* Letting the user generate e.g. 8 words, and select the 6 they like best.
* Letting the user generate e.g. four sets of 6 words, and choose the set they like the best.

Both of these would further improve the user's ability to make memorable passphrases.  But both of them would also negatively impact entropy.  I just don't know by how much.  And it needs to be quantifiable to make it acceptable from a security perspective.

The problem basically comes down to how to mathematically model the entropy in a conservative way.  The model needs to adhere to "the user always does the worst possible thing, and the adversary knows that".  At the moment, it's not even clear to me what the "worst possible thing" is in either of these two cases.

If anyone has any insight into this, I'd love to hear about it!  I'm really hoping it's one of those things where it's already been solved, and I just don't know where to look for the answers.
