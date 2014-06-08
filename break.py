#!/usr/bin/env python
# -*- encoding: utf-8 -*-

import os
import datetime
import shutil

extensions = ['jpg', 'jpeg', 'png', 'gif']

for filename in os.listdir('.'):
    matches = [filename.lower().endswith(ext) for ext in extensions]
    if not any(matches):
        continue

    stat = os.stat(filename)
    mod_date = datetime.datetime.fromtimestamp(stat.st_mtime)
    # print '{filename} modified in {date}'.format(
    #     filename=filename,
    #     date=mod_date)
    directory = os.path.join('{year:0>4}'.format(year=mod_date.year),
                             '{month:0>2}'.format(month=mod_date.month))
    print '{filename} will be moved to {directory}'.format(
        filename=filename,
        directory=directory)

    try:
        os.makedirs(directory)
    except OSError:
        pass

    shutil.move(filename, os.path.join(directory, filename))
