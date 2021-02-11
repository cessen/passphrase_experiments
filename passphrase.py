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

    percent_of_usable_words = 100
    if len(sys.argv) >= 4:
        percent_of_usable_words = int(sys.argv[3])

    # Parse the word list file
    words = parse_wordlist_file(wordlist_filename)
    
    apprx_word_list_length = len(words) * percent_of_usable_words // 100

    # Estimated entropy.
    entropy = int(math.floor(math.log(apprx_word_list_length**passphrase_length, 2)))

    # Estimated entropy if the words are rearranged.
    entropy_rearranged = int(math.floor(math.log(cmbn(apprx_word_list_length, passphrase_length), 2)))

    passphrase = ""

    for i in range(0, passphrase_length):
        passphrase += secrets.choice(words) + " "

    print(passphrase)
    print("Word list size: " + str(len(words)))
    print("Estimated entropy: " + str(entropy) + " bits")
    print("Estimated entropy with rearrangement: " + str(entropy_rearranged) + " bits")

