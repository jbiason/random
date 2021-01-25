import random
import xml.etree.ElementTree as ET
from urllib.parse import unquote

tree = ET.parse('ebooks2.xml')
root = tree.getroot()

favs = []

for response in root:
    url = response.find('{DAV:}href').text
    favorite = response.find('{DAV:}propstat')\
          .find('{DAV:}prop')\
          .find('{http://owncloud.org/ns}favorite').text

    if favorite != '1':
        continue

    quoted = url.split('/')[-2]
    name = unquote(quoted)
    favs.append(name)

print(random.choice(favs))
