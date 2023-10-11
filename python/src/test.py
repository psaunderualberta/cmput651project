import libcmput651py
import numpy as np
import pandas as pd
# import matplotlib.pyplot as plt
import pickle
from datetime import datetime
import os

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

results = []
seconds_per_hour = 60 * 60
for seed in range(8):
    sim_result = libcmput651py.alife.simulation("den312d", seed, seconds_per_hour)
    python_sim_result = {
        "best": heuristic2dict(sim_result.best),
        "heuristics": list(map(heuristic2dict, sim_result.heuristics))
    }

    results.append({
        "seed": seed,
        "result": python_sim_result
    })

# Print total # of heuristics
print(sum(map(lambda t: len(t["result"]["heuristics"]), results)))

today = datetime.today().strftime("%m-%d-%y")
fname = os.path.join("..", "python", "out", f"{today}.pickle")
with open(fname, "wb") as f:
    pickle.dump(results, f, pickle.HIGHEST_PROTOCOL)
