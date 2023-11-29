from libcmput651py import genetic_algorithm as ga
from libcmput651py import get_problems
import random
import numpy as np
import time


def exponential(n, gamma):
    arr = np.zeros(n, dtype=np.float64)
    x = 1.0
    for i in range(n):
        arr[i] = x
        x = x * gamma
    arr = arr / np.sum(arr)
    return arr


GA_PERF_WEIGHTS = exponential(10, 0.8)


def get_individuals(population):
    return [ga.probabilities2dict(p) for p in population]


class MetaIndividual:
    def __init__(self, probs, fitness=None):
        self.probs = probs
        self.fitness = fitness

    def update_fitness(self, fitness):
        if self.fitness is None:
            self.fitness = fitness
        else:
            gamma = 0.5
            self.fitness = gamma * self.fitness + (1 - gamma) * fitness

    def mutate(self, mutation_probability):
        return MetaIndividual(ga.mutate_probabilities(self.probs, mutation_probability), self.fitness)

    def __add__(self, other):
        return MetaIndividual(ga.crossover_probabilities(self.probs, other.probs), (self.fitness + other.fitness) / 2)


def main(mutate: bool, random_init: bool):
    ga_perf_history = []

    start_time = time.time()

    print("Starting GA at {}".format(start_time))
    print("Mutate: {}".format(mutate))
    print("Random: {}".format(random_init))

    config = {
        "MAP_NAME": "maze2",
        "POPULATION_SIZE": 50,
        "SECONDS_PER_GA": 30,
        "GA_SEED": 42,
        "MUTATION_PROBABILITY": 0.1,
        "DEBUG": False,
        "FITNESS_LOG": f"../python/out/{int(start_time)}-data.csv",
        "PROB_LOG": f"../python/out/{int(start_time)}-log.csv",
        "HEURISTIC_LOG": f"../python/out/{int(start_time)}-heuristic.csv",
        "PERF_LOG": f"../python/out/{int(start_time)}-perf.csv",
        "TIMEOUT": 30 * 60,
    }

    # Create log files
    with open(config["FITNESS_LOG"], "w") as f_log:
        f_log.write("Generation | Heuristics | Fitnesses\n")
    with open(config["PROB_LOG"], "w") as p_log:
        p_log.write(
            "Generation | Population Index | Term Category | Term | Individual\n"
        )
    with open(config["HEURISTIC_LOG"], "w") as h_log:
        h_log.write(
            "Heuristics:\n"
        )

    # Get the map & baseline cycle
    rust_map, cycle = get_problems(config["MAP_NAME"])

    # Create the population
    if random_init:
        population = [
            MetaIndividual(ga.random_term_probabilities(False)) for _ in range(config["POPULATION_SIZE"])
        ]
    else:
        population = [
            MetaIndividual(ga.random_term_probabilities(True)) for _ in range(config["POPULATION_SIZE"])
        ]

    # population = [MetaIndividual(ga.random_term_probabilities(False))] * 30
    # for individual in population:
    #     print(ga.probabilities2dict(individual.probs))

    heuristic_ga = ga.get_genetic_algorithm()
    heuristic_ga.initialize_ga()
    print("Initialized GA")

    # While the budget has not been reached
    gen_num = 0
    # while time.time() - start_time < config["TIMEOUT"]:
    while gen_num < 100:
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
            for i, term in enumerate(get_individuals(list(map(lambda x: x.probs, population)))):
                for term_type, probs in term.items():
                    for term, prob in probs.items():
                        log_file.write(f"{gen_num} | {i} | {term_type} | {term} | {prob}\n")

        # Evaluate the population
        # heuristic_fitnesses = [
        #     ga.genetic_algorithm(
        #         rust_map, cycle, probs, config["GA_SEED"], config["SECONDS_PER_GA"]
        #     )
        #     for probs in population
        # ]

        best_individuals, prob_perf = heuristic_ga.step_with_probs(list(map(lambda x: x.probs, population)))
        # print("Population: {}".format(population))
        # print("Prob performance: {}".format(prob_perf))
        print("Best individuals: {}".format(best_individuals))

        # ga_perf = np.dot(GA_PERF_WEIGHTS, np.array([p[1] for p in best_individuals]))
        ga_perf = best_individuals[0][1]
        ga_perf_history.append(ga_perf)
        print("GA performance: {}".format(ga_perf))

        # save the heuristics
        with open(config["HEURISTIC_LOG"], "a") as log_file:
            log_file.write(f"{gen_num} | {best_individuals}\n")

        # # Extract the heuristics and fitnesses
        # all_heuristics = tuple(
        #     tuple(hf[0] for hf in heuristic_fitness)
        #     for heuristic_fitness in heuristic_fitnesses
        # )
        # all_fitnesses = tuple(
        #     tuple(hf[1] for hf in heuristic_fitness)
        #     for heuristic_fitness in heuristic_fitnesses
        # )

        # # Write all_heuristics and all_fitnesses to log file
        # with open(config["FITNESS_LOG"], "a") as log_file:
        #     log_file.write(f"{gen_num} | ")
        #     log_file.write(f"{all_heuristics} | ")
        #     log_file.write(f"{all_fitnesses}\n")

        # Calculate the fitnesses as the mean of the heuristic fitnesses
        # fitnesses = np.mean(all_fitnesses, axis=1)

        for i, (individual, fitness) in enumerate(zip(population, prob_perf)):
            individual.update_fitness(fitness)
            population[i] = individual

        fitnesses = list(map(lambda x: x.fitness, population))

        assert all([f >= 0 for f in fitnesses])

        # Crossover the parents probabilistically w.r.t. fitness
        if mutate:
            first_parents = random.choices(
                population, weights=fitnesses, k=config["POPULATION_SIZE"]
            )
            second_parents = random.choices(
                population, weights=fitnesses, k=config["POPULATION_SIZE"]
            )
            children = [
                p1 + p2 for p1, p2 in zip(first_parents, second_parents)
            ]

            # Mutate the children
            population = [
                c.mutate(config["MUTATION_PROBABILITY"]) for c in children
            ]
            if config["DEBUG"]:
                for c, p in zip(children, population):
                    print("Binary: {} -> {}".format(c.binaries, p.binaries))
                    print("Unaries: {} -> {}".format(c.unaries, p.unaries))
                    print("Terminals: {} -> {}".format(c.terminals, p.terminals))
                    print("Numbers: {} -> {}".format(c.numbers, p.numbers))

        # Increment the generation
        gen_num += 1

    print("GA performance history: {}".format(ga_perf_history))

    # Write the GA performance history to a file
    with open(config["PERF_LOG"], "w") as log_file:
        # write settings
        log_file.write(f"{config}\n")
        # write arguments
        log_file.write(f"{mutate} | {random_init}\n")
        # write performance history
        log_file.write(f"{ga_perf_history}\n")


if __name__ == "__main__":
    for i in range(10):
        main(True, True)
    # for i in range(10):
        # main(False, False)
    # for i in range(10):
    #     main(False, True)
