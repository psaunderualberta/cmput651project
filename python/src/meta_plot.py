import sys
import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import scienceplots

FILE_LOCATION = os.path.dirname(os.path.realpath(__file__))
__aspect_ratio = 6/8
WIDTH = 3.31314  # width of one column in latex twocolumn
HEIGHT = WIDTH * __aspect_ratio


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
    plt.figure(figsize=(WIDTH, HEIGHT))
    plt.plot(champions)
    plt.xlabel("Generation")
    plt.ylabel("Champion")
    plt.title("Champion cost over time for Meta-GA on maze2")
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
    plt.figure(figsize=(WIDTH, HEIGHT))
    plt.grid(True)
    plt.scatter(x, avg_fitnesses, alpha=0.5, linewidths=0.5, s=1)
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
    plt.title("All fitnesses for Meta-GA on maze2")
    plt.savefig(out_path)
    plt.close()


def plot_genetic_algorithms(fitness_log, out_path):
    # Extract the history
    df = pd.read_csv(fitness_log, sep="|", engine="python")
    df.columns = [c.strip() for c in df.columns]
    df = df.dropna()
    
    # Find the sequence of best fitnesses in each generation
    groups = df.groupby(["Meta-Generation", "Population Member", "Generation"])
    best_fitness_per_population = groups[["Fitness", "Unix Time"]].min().reset_index()
    gb = best_fitness_per_population.groupby(["Meta-Generation", "Population Member"])
    fitnesses = gb["Fitness"].apply(list).reset_index()
    unix_timestamps = gb["Unix Time"].apply(np.array).reset_index()
    timestamps = unix_timestamps["Unix Time"].apply(lambda x: pd.to_datetime(x, unit="s", origin="unix")).reset_index()
    timestamps = timestamps["Unix Time"].apply(lambda x: x - min(x)).reset_index()
    cumulative_best_fitnesses = fitnesses["Fitness"].apply(lambda x: pd.Series(x).cummin().to_numpy())

    # Plot the fitnesses
    plt.figure(figsize=(WIDTH, HEIGHT))
    for x, y in zip(timestamps["Unix Time"], cumulative_best_fitnesses):
        x = x / np.timedelta64(1, "h")
        plt.plot(x, y, alpha=1, linewidth=0.5)

    # Average the curves
    x, y, std = __average_curves(timestamps["Unix Time"], cumulative_best_fitnesses)
    plt.plot(x / np.timedelta64(1, "h"), y, linewidth=1.5, color="black", label="Average")
    plt.fill_between(
        x / np.timedelta64(1, "h"),
        y - std,
        y + std,
        alpha=0.3,
        color="black",
        label="Standard Deviation",
    )


    plt.xlabel("Hours of Synthesis")
    plt.ylabel("Lowest cost encountered")
    plt.title("12 Different Mutation Probabilities : hrt201d")
    plt.legend()
    plt.savefig(out_path)
    plt.close()


def __average_curves(x, y, step="1s"):
    """
    Average the curves, return also the standard deviation.
    Note that the lengths of all curves is not the same,
    and the x-values do not necessarily match.
    'x' is of pandas series of datetimes, y is a list of floats
    """

    # Resample the curves
    resampled_x = []
    resampled_y = []
    for i, (x_i, y_i) in enumerate(zip(x, y)):
        df = pd.DataFrame({"x": x_i, "y": y_i})
        resampled = df.resample(step, on="x")
        resampled_x.append(resampled.mean().ffill().reset_index()["x"])
        resampled_y.append(resampled.mean().ffill().reset_index()["y"].astype(float))

    # Concatenate the resampled curves
    df = pd.DataFrame({
        "x": pd.concat(resampled_x).reset_index(drop=True),
        "y": pd.concat(resampled_y).reset_index(drop=True),
    })

    # Average the curves
    grouper = pd.Grouper(key="x", freq=step)
    resampled = df.groupby(grouper)
    new_x = resampled.mean().reset_index()["x"]
    new_y = resampled.mean().reset_index()["y"].astype(float)
    new_std = resampled.std().reset_index()["y"]

    return new_x, new_y, new_std

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
    plt.style.use('science')
    # plot_fitnesses(results, os.path.join(plot_path, f"{file_id}-fitnesses.pdf"))
    # plot_champions(results, os.path.join(plot_path, f"{file_id}-champion.pdf"))
    plot_genetic_algorithms(genetic_algorithms, os.path.join(plot_path, f"{file_id}-genetic_algorithms.pdf"))

if __name__ == "__main__":
    main()