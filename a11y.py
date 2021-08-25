import sys

phrase = sys.argv[1]
for word in phrase.split():
    print('{initial}{size}{final} '.format(
        initial=word[0],
        final=word[-1],
        size=max(len(word)-2, 0)
    ), end='')

print('')
