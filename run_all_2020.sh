#!/bin/bash

for x in `seq -f %02.f 1 25`
do
echo "*** day ${x} ***"
python -m advent_2020.day_${x}
done