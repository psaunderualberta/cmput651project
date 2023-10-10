import libcmput651py
import numpy as np
import pandas as pd
# import matplotlib.pyplot as plt

print(libcmput651py.sum_as_string(1, 2))

manhattan_distance_heuristic = libcmput651py.heuristic.manhattan_distance()
print(type(manhattan_distance_heuristic))
print(manhattan_distance_heuristic)

libcmput651py.test_heuristic(manhattan_distance_heuristic)
print(dir(libcmput651py))
results = libcmput651py.solve_cycle_on_map("den312d", manhattan_distance_heuristic)
num_expansions = list(map(lambda x: len(x.expansions), results))
traversals = list(map(lambda x: x.num_traversals, results))

