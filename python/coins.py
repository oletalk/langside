import random

coins = [1, 2, 2, 5, 10, 10, 20, 20, 100]
ls = len(coins)
num_of_coins = random.randint(2, ls-1)
total = 0

for x in range(0, num_of_coins):
    position = random.randrange(0, len(coins))
    total += coins.pop(position)

print("Try making {}p with {} coins.".format(total, num_of_coins))

