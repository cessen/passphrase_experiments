## Passphrase Generation Experiments

Experiments in passphrase generation.

Usage:

```
passphrase.py word_list.txt <passphrase_length>
```

You can pass any newline-separated word list you like, and the passphrase length is specified in number of words.

For generating actual passphrases, I recommend using your own (secure) source of random numbers to choose your words.  I do utilize python's `secrets` library to select the words, which in theory should be secure.  But I have no idea if there are special considerations when using that API, so trust my code at your own risk.

The more interesting thing here is the word list and the entropy estimates.

Here is some example output from the program:

```
$ ./passphrase.py word_list.txt 6
tool rave pond chug juice stow
Word list size: 2499
Estimated entropy: 67 bits
Estimated entropy with rearrangement: 58 bits
```

There are two listed entropy estimates: one for using the words in the given order, and one for if you decide to rearrange them to your liking.  The main experiment of this repo is the latter: can longer passphrases be made more memorable and viable by allowing the user to rerrange the words into a more memorable order?

For example, using the words above, we could rearrange them like this:

```
stow pond tool, chug rave juice
```

Now it reads like a (strange) task list.  We've lost 9 bits of secure entropy this way, but we've also made it *far* more memorable.  And being able to rearrange the words makes it (hopefully) more practical to use even longer pass phrases.

The other thing you can do with this word list is change the form of the words.  Let's take a look at another example:

```
./passphrase.py word_list.txt 7
dash copy jam smug yield grid revel 
Word list size: 2499
Estimated entropy: 79 bits
Estimated entropy with rearrangement: 66 bits
```

We can turn it into this:

```
smug jam revels in copying dashed yield grid
```

Aside from the added "in", the *form* of some words has changed.  For example, "copy" -> "copying".

With many word lists this would be a no-no, as those alternate forms might *also* exist in the list, so you would be reducing the entropy of your passphrase--potentially significantly.  However, `word_list.txt` in this repo has been hand-curated to avoid multiple forms of the same word.  I don't claim that it's perfect--I may have missed a few words here and there.  But it should mostly be free of multiple forms, making it safe to change those forms as you please.

In the end, I don't know if either of these things *actually* provide much practical benefit.  But I thought it was worth experimenting with.
