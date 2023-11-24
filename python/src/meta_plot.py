import sys
import os
import pandas as pd
import numpy as np
# import matplotlib.pyplot as plt


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
    best_fitnesses = np.max(avg_fitnesses, axis=1)
    champions = pd.Series(best_fitnesses).cummax().to_numpy()

    # Plot the fitnesses
    # plt.plot(fitnesses)
    # plt.xlabel("Generation")
    # plt.ylabel("Champion")
    # plt.savefig(out_path)


def plot_fitnesses(fitness_log, out_path):
    """Plot the fitnesses from the fitness log."""
    # Extract the fitnesses
    _, fitnesses = extract_data(fitness_log)

    # Compute the average fitnesses for each individual in each population
    avg_fitnesses = np.mean(fitnesses, axis=2)
    std_fitnesses = np.std(fitnesses, axis=2)

    # Plot the fitnesses
    # plt.plot(avg_fitnesses)
    # plt.xlabel("Generation")
    # plt.ylabel("Fitness")
    # plt.savefig(out_path)


def main():
    """Plot the fitnesses from the fitness log."""
    # Parse the command line arguments
    assert len(sys.argv) == 2, "Usage: python src/meta_plot.py <file_id>"
    file_id = sys.argv[1]
    results = os.path.join(FILE_LOCATION, "..", "out", f"{file_id}-data.csv")
    probs = os.path.join(FILE_LOCATION, "..", "out", f"{file_id}-log.csv")

    plot_path = os.path.join(FILE_LOCATION, "..", "plots")
    if not os.path.exists(plot_path):
        os.makedirs(plot_path)

    # Plot the fitnesses
    plot_fitnesses(results, os.path.join(plot_path, f"{file_id}-champion.png"))
    plot_champions(results, os.path.join(plot_path, f"{file_id}-fitnesses.png"))

if __name__ == "__main__":
    main()