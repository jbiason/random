import sqlite3
import itertools

names = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight',
         'nine', 'ten', 'eleven', 'twelve', 'thirteen', 'fourteen', 'fifteen',
         'sixteen', 'seventeen', 'eighteen', 'nineteen', 'twenty']
groups = itertools.cycle([1, 2, 3])
ids = itertools.count(1)

data = zip(ids, names, groups)

con = sqlite3.connect(':memory:')
cur = con.cursor()
cur.execute('''create table lite (id int, name text, grp int)''')
cur.executemany('INSERT INTO lite values (?,?,?)', data)

filters = (1,3)
query = 'select name, grp from lite where grp in ({})'.format(
    ', '.join('?' * len(filters)))
print(query)

for record in cur.execute(query, filters).fetchall():
    print(record)
