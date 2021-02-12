## Passphrase Generation Experiments

Experiments in passphrase generation.

Usage:

```
passphrase -l <passphrase_length>
```

The passphrase length is specified in number of words.  The source word list is `word_list.txt` in the root of this repo.

For generating actual passphrases, I recommend using your own (secure) source of random numbers to choose your words.  Trust this software's randomness at your own risk.

The more interesting thing here is the word list itself and the entropy estimates.

Here is some example output from the program:

```
$ ./passphrase -l 6
tool rave pond chug juice stow

Entropy in this order:  70 bits
Entropy rearranged:     60 bits
```

There are two listed entropy estimates: one for using the words in the given order, and one for if you decide to rearrange them to your liking.  The main experiment of this repo is the latter: can longer passphrases be made more memorable and practical by allowing the user to rearrange the words into a more memorable order?

For example, using the words above, we could rearrange them like this:

```
stow pond tool, chug rave juice
```

Now it reads like a (strange) task list.  We've lost 10 bits of secure entropy this way, but we've also made it *far* more memorable.  And being able to rearrange the words makes it (hopefully) more practical to use even longer pass phrases.

The other thing you can do with this word list is change the form of the words.  Let's take a look at another example:

```
$ ./passphrase -l 7
dash copy jam smug yield grid revel 

Entropy in this order:  81 bits
Entropy rearranged:     69 bits
```

We can turn it into this:

```
smug jam revels in copying dashed yield grid
```

Aside from the added "in", the *form* of some words has changed.  For example, "copy" -> "copying".

With many word lists this would be a no-no, as those alternate forms might *also* exist in the list, so you would be reducing the entropy of your passphrase--potentially significantly.  However, `word_list.txt` in this repo has been hand-curated to avoid multiple forms of the same word.  I don't claim that it's perfect--I may have missed a few words here and there.  But it should mostly be free of multiple forms, making it safe to change those forms as you please.

In the end, I don't know if either of these things *actually* provide much practical benefit.  But I thought it was worth experimenting with.
