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
Healthy poet hands.  A sauce truce exists.
```

The capitalization and punctuation are superfluous.  Also, thanks to the design of the source word list, the changed word forms and the additional word "a" are also superfluous, and don't affect the entropy.

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

Both of these would further improve the user's ability to make memorable passphrases.  But both of them would also negatively impact security.  I just don't know by how much.  And it needs to be quantifiable to make it acceptable from a security perspective.

An important aspect of modeling the security margins with these additional ideas is that simply modeling the number of possible resulting passwords isn't (I suspect) enough.  Both of the approaches above will likely change the probability distribution of the passwords as well.  For example, some passwords will become *more likely* and others *less likely*, even if very few become strictly impossible.  And those changing probabilities would give an adversary an edge on guessing passwords.

As a starting point, I think the following (or something equivalent) is a good model:

1. The password system has two adversaries who are collaborating: a traditional adversary who is trying to guess the password, and a spy "user" who also wants the adversary to guess their password.
2. These adversaries have perfect knowledge of the password generation scheme, but are limited to working within its rules.
3. They can agree ahead of time on a strategy.
4. After they agree on a strategy, they can no longer communicate, and the spy-user creates their password.

I believe this effectively models a combination of worst-possible-user and strongest-possible-adversary, which lets us give lower bounds on security if we're able to prove anything about it for a given password scheme.

With a 100% random passphrase, the user can't intervene at all, so it's trivial to analyze.  And even if rearrangement is allowed, the best adversarial strategy is fairly obvious: sort the words of the generated password, making their order 100% predictable and thus irrelevant.  This is also easy to analyze with a bit of combinatorial math.

However, with the two additional schemes above, it's not at all clear to me what the best adversarial strategy even is.  I can certainly think of strategies that would make cracking easier (e.g. ranking all words in the source word list, and always choosing the passphrase with the lowest summed rank).  But I don't know if any of them are the *best* adversarial strategy.

If anyone has any insight into this, I'd love to hear about it!  I'm really hoping it's one of those things where it has already been solved, and I just don't know where to look for the answers.
