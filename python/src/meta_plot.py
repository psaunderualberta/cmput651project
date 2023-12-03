import sys
import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt


FILE_LOCATION = os.path.dirname(os.path.realpath(__file__))


def extract_data(fitness_log):
    """Extract the data from the fitness log."""
    # Read the fitness log
    df = pd.read_csv(fitness_log, sep="|", engine="python", dtype=str)
    df = df.dropna()

    # Clean the data
    df.columns = [c.strip() for c in df.columns]
    df = df.apply(lambda x: x.str.strip() if x.dtype == "object" else x)

    # Extract the fitnesses. The list call is necessary for some reason.
    fitnesses = np.array(list(df["Fitnesses"].apply(lambda x: np.array(eval(x)))))
    heuristics = np.array(list(df["Heuristics"].apply(lambda x: np.array(eval(x)))))

    return heuristics, fitnesses


def plot_champions(fitness_log, out_path):
    """Plot the fitnesses from the fitness log."""
    # Extract the fitnesses
    _, fitnesses = extract_data(fitness_log)

    # Compute the average fitnesses for each individual in each population
    avg_fitnesses = np.mean(fitnesses, axis=2)
    std_fitnesses = np.std(fitnesses, axis=2)

    # Get the champion fitnesses
    best_fitnesses = np.min(avg_fitnesses, axis=1)
    champions = pd.Series(best_fitnesses).cummin().to_numpy()

    # Plot the fitnesses
    plt.figure(figsize=(25, 12))
    plt.plot(champions)
    plt.xlabel("Generation")
    plt.ylabel("Champion")
    plt.title("Champion cost over time for maze2")
    plt.savefig(out_path)
    plt.close()


def plot_fitnesses(fitness_log, out_path):
    """Plot the fitnesses from the fitness log."""
    # Extract the fitnesses
    _, fitnesses = extract_data(fitness_log)

    # Compute the average fitnesses for each individual in each population
    avg_fitnesses = np.mean(fitnesses, axis=2)
    avg_fitness_by_gen = np.mean(avg_fitnesses, axis=1)
    std_fitness_by_gen = np.std(avg_fitnesses, axis=1)

    # Compute the x & y coordinates
    _, x = np.meshgrid(np.arange(avg_fitnesses.shape[1]), np.arange(avg_fitnesses.shape[0]))

    # Plot the fitnesses
    plt.figure(figsize=(25, 12))
    plt.grid(True)
    plt.scatter(x, avg_fitnesses, alpha=0.5, s=2)
    plt.plot(avg_fitness_by_gen, color="black")
    plt.fill_between(
        np.arange(avg_fitness_by_gen.shape[0]),
        avg_fitness_by_gen - std_fitness_by_gen,
        avg_fitness_by_gen + std_fitness_by_gen,
        alpha=0.2,
        color="black",
    )
    plt.xlabel("Generation")
    plt.ylabel("Fitness")
    plt.title("All costs and their average + standard deviation for maze2")
    plt.savefig(out_path)
    plt.close()


def plot_genetic_algorithms(fitness_log, out_path):
    # Extract the history
    df = pd.read_csv(fitness_log, sep="|", engine="python")
    df.columns = [c.strip() for c in df.columns]
    df = df.dropna()
    
    # Find the sequence of best fitnesses in each generation
    groups = df.groupby(["Meta-Generation", "Population Member", "Generation"])
    best_fitness_per_population = groups["Fitness"].min().reset_index()
    all_fitnesses_in_ga = best_fitness_per_population.groupby(["Meta-Generation", "Population Member"])["Fitness"].apply(list).reset_index()
    cumulative_best_fitnesses = all_fitnesses_in_ga["Fitness"].apply(lambda x: pd.Series(x).cummin().to_numpy())

    # Plot the fitnesses
    plt.figure(figsize=(25, 12))
    for row in cumulative_best_fitnesses:
        approx_hours = np.linspace(0, 1, len(row))
        plt.plot(approx_hours, row)

    plt.xlabel("Approximate hours of Synthesis")
    plt.ylabel("Lowest cost encountered")
    plt.title("Lowest cost encountered in each generation for synthesis on hrt201d")
    plt.savefig(out_path)
    plt.close()

def main():
    """Plot the fitnesses from the fitness log."""
    # Parse the command line arguments
    assert len(sys.argv) == 2, "Usage: python src/meta_plot.py <file_id>"
    file_id = sys.argv[1]
    results = os.path.join(FILE_LOCATION, "..", "out", f"{file_id}-data.csv")
    probs = os.path.join(FILE_LOCATION, "..", "out", f"{file_id}-log.csv")
    genetic_algorithms = os.path.join(FILE_LOCATION, "..", "out", f"{file_id}-history.csv")


    plot_path = os.path.join(FILE_LOCATION, "..", "plots")
    if not os.path.exists(plot_path):
        os.makedirs(plot_path)

    # Plot the fitnesses
    plt.rc('font', size=25)
    plt.rc('xtick', labelsize=20)
    plt.rc('ytick', labelsize=20)
    plot_fitnesses(results, os.path.join(plot_path, f"{file_id}-champion.png"))
    plot_champions(results, os.path.join(plot_path, f"{file_id}-fitnesses.png"))
    # plot_genetic_algorithms(genetic_algorithms, os.path.join(plot_path, f"{file_id}-genetic_algorithms.png"))

if __name__ == "__main__":
    main()