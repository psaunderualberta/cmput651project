from libcmput651py import genetic_algorithm as ga
from libcmput651py import get_problems
import random

def main():
    config = {
        "MAP_NAME": "den312d",
        "POPULATION_SIZE": 10,
        "SECONDS_PER_GA": 2,
        "GA_SEED": 42,
        "MUTATION_PROBABILITY": 0.1,
        "DEBUG": True,
    }

    # Get the map & baseline cycle
    rust_map, cycle = get_problems(config["MAP_NAME"])

    # Create the population
    population = [ga.random_term_probabilities(False) for _ in range(config["POPULATION_SIZE"])]

    # While the budget has not been reached
    gen_num = 0
    while gen_num < 1:
    
        # Evaluate the population
        fitness = [ga.genetic_algorithm(rust_map, cycle, probs, config["GA_SEED"], config["SECONDS_PER_GA"]) for probs in population]
        assert(all([f >= 0 for f in fitness]))
        # fitness = [random.random() for _ in range(config["POPULATION_SIZE"])]
    
        # Crossover the parents probabilistically w.r.t. fitness        
        first_parents = random.choices(population, weights=fitness, k=config["POPULATION_SIZE"])
        second_parents = random.choices(population, weights=fitness, k=config["POPULATION_SIZE"])
        children = [ga.crossover_probabilities(p1, p2) for p1, p2 in zip(first_parents, second_parents)]

        # Mutate the children
        population = [ga.mutate_probabilities(c, config["MUTATION_PROBABILITY"]) for c in children]
        if config["DEBUG"]:
            for c, p in zip(children, population):
                print("Binary: {} -> {}".format(c.binaries, p.binaries))
                print("Unaries: {} -> {}".format(c.unaries, p.unaries))
                print("Terminals: {} -> {}".format(c.terminals, p.terminals))
                print("Numbers: {} -> {}".format(c.numbers, p.numbers))

        # Increment the generation
        gen_num += 1

    # Log final data

if __name__ == '__main__':
    main()