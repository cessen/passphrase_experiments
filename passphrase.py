#!/usr/bin/env python3

import sys
import os
import re
import math
import secrets



def parse_wordlist_file(path):
    words = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if re.match("^[a-z]+$", line):
                words += [line]
    return words

# The combination function.
def cmbn(n, k):
    if k == 0:
        return 1
    c = n
    for i in range(2, k + 1):
        n -= 1
        c *= n
        c //= i
    return c


if __name__ == "__main__":
    wordlist_filename = sys.argv[1]
    passphrase_length = int(sys.argv[2])

    words_to_generate = passphrase_length
    if len(sys.argv) >= 4:
        words_to_generate = max(words_to_generate, int(sys.argv[3]))

    # Parse the word list file
    words = parse_wordlist_file(wordlist_filename)
    
    effective_word_list_length = len(words) * passphrase_length // words_to_generate

    # Estimated entropy.
    entropy = int(math.floor(math.log(effective_word_list_length**passphrase_length, 2)))

    # Estimated entropy if the words are rearranged.
    entropy_rearranged = int(math.floor(math.log(cmbn(effective_word_list_length, passphrase_length), 2)))

    passphrase = ""

    for i in range(0, words_to_generate):
        passphrase += secrets.choice(words) + " "

    print(passphrase)
    if passphrase_length < words_to_generate:
        print("^^^ please select {} words ^^^".format(passphrase_length))
    print("")
    print("Source word list size: " + str(len(words)))
    print("Estimated entropy with same word order:  " + str(entropy) + " bits")
    print("Estimated entropy with rearrangement:    " + str(entropy_rearranged) + " bits")

