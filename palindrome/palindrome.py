#!/usr/bin/env python
# -*- encoding: utf-8 -*-

"""Finds the largest palindrome in a string."""

import string
import re

def largest(phrase):
    """Return the largest palindrome inside the phrase."""
    regex = re.compile('[{punctuation}]'.format(
        punctuation=string.punctuation))
    clean_phrase = regex.sub('', phrase).replace(' ', '')
    reverse_phrase = clean_phrase[::-1]

    # print 'Clean:', clean_phrase
    # print 'Reverse:', reverse_phrase

    max_palindrome = None
    for pos in range(3, len(clean_phrase) + 1):
        # print pos,
        if reverse_phrase[:pos] in clean_phrase:
            # print 'yes'
            max_palindrome = pos
        else:
            # print 'no'
            break

    return max_palindrome


def main():
    """Main code."""
    phrase = raw_input('Phrase: ')
    large = largest(phrase)
    print 'Max palindrome found:', large


if __name__ == '__main__':
    main()
