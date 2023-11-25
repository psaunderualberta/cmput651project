from libcmput651py import genetic_algorithm as ga
from libcmput651py import get_problems
import random
import numpy as np
import time


def get_individuals(population):
    return [ga.probabilities2dict(p) for p in population]


def main():
    start_time = time.time()

    config = {
        "MAP_NAME": "maze2",
        "POPULATION_SIZE": 12,
        "SECONDS_PER_GA": 60 * 60,
        "GA_SEED": 42,
        "MUTATION_PROBABILITY": 0.1,
        "DEBUG": False,
        "BEST_LOG": f"../python/out/{int(start_time)}-data.csv",
        "PROB_LOG": f"../python/out/{int(start_time)}-log.csv",
        "HISTORY_LOG": f"../python/out/{int(start_time)}-history.csv",
        "TIMEOUT": 1,
    }

    # Create log files
    with open(config["BEST_LOG"], "w") as f_log:
        f_log.write("Generation | Heuristics | Fitnesses\n")
    with open(config["PROB_LOG"], "w") as p_log:
        p_log.write("Generation | Population Index | Term Category | Term | Individual\n")
    with open(config["HISTORY_LOG"], "w") as h_log:
        h_log.write("Meta-Generation | Population Member | Generation | Heuristic | Fitness\n")

    # Get the map & baseline cycle
    rust_map, cycle = get_problems(config["MAP_NAME"])

    # Create the population
    population = [
        ga.random_term_probabilities(False) for _ in range(config["POPULATION_SIZE"])
    ]

    # While the budget has not been reached
    gen_num = 0
    while time.time() - start_time < config["TIMEOUT"]:
        current_time = time.time()
        # Print the generation number, elapsed time, and time remaining
        print(
            "Generation {} | {:0.2f} seconds elapsed | {:0.2f} seconds remaining".format(
                gen_num,
                current_time - start_time,
                config["TIMEOUT"] - (current_time - start_time),
            )
        )

        with open(config["PROB_LOG"], "a") as log_file:
            for i, term in enumerate(get_individuals(population)):
                for term_type, probs in term.items():
                    for term, prob in probs.items():
                        log_file.write(f"{gen_num} | {i} | {term_type} | {term} | {prob}\n")

        # Evaluate the population
        results = [
            ga.genetic_algorithm(
                rust_map, cycle, probs, config["GA_SEED"], config["SECONDS_PER_GA"]
            )
            for probs in population
        ]

        # Write the history of the genetic algorithms to history_log
        with open(config["HISTORY_LOG"], "a") as file:
            for pop_i, result in enumerate(results):
                for gen_i, generation in enumerate(result.history):
                    for h, f in generation:
                        file.write(f"{gen_num} | {pop_i} | {gen_i} | \"{h}\" | {f}\n")


        # Extract the heuristics and fitnesses
        best_heuristics = tuple(
            tuple(h for h in result.best_heuristics)
            for result in results
        )
        best_fitnesses = tuple(
            tuple(f for f in result.best_fitnesses)
            for result in results
        )

        # Write best_heuristics and best_fitnesses to log file
        with open(config["BEST_LOG"], "a") as log_file:
            log_file.write(f"{gen_num} | {best_heuristics} | {best_fitnesses}\n")

        # Calculate the fitnesses as the mean of the heuristic fitnesses
        fitnesses = np.mean(best_fitnesses, axis=1)
        assert all([f >= 0 for f in fitnesses])

        # Crossover the parents probabilistically w.r.t. fitness
        first_parents = random.choices(
            population, weights=fitnesses, k=config["POPULATION_SIZE"]
        )
        second_parents = random.choices(
            population, weights=fitnesses, k=config["POPULATION_SIZE"]
        )
        children = [
            ga.crossover_probabilities(p1, p2)
            for p1, p2 in zip(first_parents, second_parents)
        ]

        # Mutate the children
        population = [
            ga.mutate_probabilities(c, config["MUTATION_PROBABILITY"]) for c in children
        ]
        if config["DEBUG"]:
            for c, p in zip(children, population):
                print("Binary: {} -> {}".format(c.binaries, p.binaries))
                print("Unaries: {} -> {}".format(c.unaries, p.unaries))
                print("Terminals: {} -> {}".format(c.terminals, p.terminals))
                print("Numbers: {} -> {}".format(c.numbers, p.numbers))

        # Increment the generation
        gen_num += 1


if __name__ == "__main__":
    main()
