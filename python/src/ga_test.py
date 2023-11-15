import libcmput651py
# import numpy as np
# import pandas as pd
# # import matplotlib.pyplot as plt
# import pickle
# from datetime import datetime
# import os

def heuristic2dict(h):
    return  {
        "heuristic": h.heuristic,
        "creation": h.creation,
        "score": h.score
    }

print(libcmput651py.sum_as_string(1, 2))

manhattan_distance_heuristic = libcmput651py.heuristic.manhattan_distance()
print(type(manhattan_distance_heuristic))
print(manhattan_distance_heuristic)

libcmput651py.test_heuristic(manhattan_distance_heuristic)
print(dir(libcmput651py))
results = libcmput651py.solve_cycle_on_map("den312d", manhattan_distance_heuristic)
num_expansions = list(map(lambda x: len(x.expansions), results))
traversals = list(map(lambda x: x.num_traversals, results))

rust_map, cycle = libcmput651py.get_problems("den312d")
print(rust_map)
print(cycle)

probs = libcmput651py.random_term_probabilities(False)
libcmput651py.genetic_algorithm.genetic_algorithm(rust_map, cycle, probs, 42, 10)

