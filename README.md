# Passphrase Generation Experiments

This repository is an experiment in letting users create more memorable passphrases.  It's in the same spirit as diceware, but with more flexibility.  The idea is to let users participate in crafting their passphrases, but in ways that can be accurately modeled to put lower bounds on the resulting security.

This repository allows three such user-interventions:

1. The user can change the form of each word.  For example, "eat" -> "ate" or "quick" -> "quickly".  This is accomplished by including only one form of each word in the source word list.
2. The user can rearrange the words however they like.  This is accomplished by calculating the entropy [without respect to word order](https://en.wikipedia.org/wiki/Combination).
3. The user can optionally choose to generate e.g. 8 words for a 6-word passphrase, discarding the excess words of their choice.  This is accomplished by calculating the [min-entropy](https://en.wikipedia.org/wiki/Min-entropy) of that model.

Together, these three capabilties give the user a lot of flexibility to craft their passphrase into something easy to memorize.  It comes at a cost to security, but it is a *known* cost, and the resulting security is reported to the user so that they can make informed decisions.


## Some examples.

To generate a 6-word passphrase, we can call the software like so:

```
$ ./passphrase -l 6
hand poet truce exist health sauce

^ Rearrange these 6 words however you like.

Min-entropy:        62 bits
Source word count:  4096
```

If these words were used in the order given, this passphrase would have 72 bits of entropy.  However, the user is encouraged to rearrange the words to make a more memorable passphrase, at the expense of 10 bits of secure entropy.  Further, they can change the form of the words, and add superfluous elements that aren't in the source word list:

```
Healthy poet hands.  A sauce truce exists.
```

The user can also choose to generate more words than are needed for the passphrase, picking the ones they want to use:

```
$ ./passphrase -l 6 -p 8
speed alien filthy gold shock maid grain rigor 

^ Pick 6 words from this list, and
  arrange them however you like.

Min-entropy:        57 bits
Source word count:  4096
```

```
Filthy alien shocks golden grain maid. 
```

This loses an additional 5 bits of secure entropy.  But it has allowed us to (on average) create an even more memorable passphrase.

Importantly, since we know the security impacts of these choices, we can compensate with longer passphrases if desired:

```
$ ./passphrase -l 7 -p 9
crude great falter apple deport musket color media either

^ Pick 7 words from this list, and
  arrange them however you like.

Min-entropy:        66 bits
Source word count:  4096
```

```
Great color media.  A crude apple musket falters.
```

Longer passphrases take longer to type and are more prone to typos, so there is certainly a trade-off there.  But the intent is to let the user weigh those trade-offs themselves.




## How to use this software.

This software depends on the security of the [getrandom](https://crates.io/crates/getrandom) crate's random numbers.  If you trust that, then you can use this software as-is.

If you don't trust that, you can instead use this software to generate a diceware list appropriate to the dice you have available, and then take the standard diceware approach with that list.

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

You can also limit the length of the words allowed in the word list (which also works when directly generating passphrases).  For example, if you want a maximum of 5-letter words:

```
$ ./passphrase -m 5 -d 6
1-1-1-1-1    abort
1-1-1-1-2    about
1-1-1-1-3    above
...
```

Note that by using these options you can end up with a list length that doesn't match the dice you have.  In fact, the only case that will match exactly is using d8 dice with the full word list.

In case of such a mismatch, follow the normal diceware procedure anyway, but for each word keep re-rolling until you get a sequence that's on the list.  This will take extra time, and you may need to re-roll quite a few times for each word depending on the extent of the mismatch.  But doing things this way should keep your word selection unbiased.


## About the word list.

The default word list used by the software is `word_list.txt` in the root of the repo.

It is very much a work-in-progress.  The goal is to avoid multiple forms of the same word, but at the moment there are almost certainly quite a few slip ups.  Moreover, what constitutes "forms" still needs to be defined to some extent.

Even with its current flaws, however, I think the list is pretty good, and should be safe to use as outlined in this readme without significant loss of security.  Neverthless, I do want to improve it further over time.  If you would like to help out with that, even just a cursory glance at some random part of `word_list.txt` would help out.

In addition to the security-oriented goals, I also have the following further usability goals for the list.  Words in the list should:

* Be easy to spell.
* Be widely known and understood (though not necessarily commonly used).
* Have meanings and nuances that lend themselves to memorable mental stories.

As far as these additional criteria go, up to this point I've just been selecting words based on how I feel about them and how I *think* other people would feel about them.  But that's obviously subject to my own biases.  Feedback on the list from this perspective is also welcome.
