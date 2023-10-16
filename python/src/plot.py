import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import os

FILENAME = os.path.join(os.path.dirname(__file__), "..", "analysis-data", "10-14-23.pickle")

def plot_best_scores_time(filename):
    alife_data = pd.read_pickle(filename)
    i = 1

    seed_data = []

    # Setting plot characteristics
    plt.figure(figsize=(10, 6))
    font = {'size': 16}
    plt.rc('font', **font)
    plt.rc('xtick', labelsize=12)
    plt.rc('ytick', labelsize=12)
    plt.xlabel('Time (Every 30s)')
    plt.ylabel('Best Heuristic Score')
    plt.title('Best Score of Heuristics Every 30s')
    plt.grid(True)

    for seed in alife_data:
        df = pd.DataFrame.from_dict(seed['result']['heuristics'])

        # Get max scores every 30s
        df.sort_values(by=['creation'])
        df['creation'] -= df['creation'][0]
        df['group'] = (df['creation'] / 30000).apply(np.floor).astype(int)
        min_scores_df = df.loc[df.groupby('group')['score'].idxmin()]
        min_scores_df['cumulative_min'] = df['score'].cummin()
        min_scores_df = min_scores_df.reset_index()
        min_scores_df.drop(columns=['score', 'heuristic'])

        seed_data.append(min_scores_df)
        min_scores_list = min_scores_df['cumulative_min'].to_numpy()
        intervals = min_scores_df['group'].to_numpy()

        plt.plot(intervals, min_scores_list, label='seed ' + str(i), linestyle='--', alpha=0.4)

        i += 1

    combined_df = pd.concat(seed_data)
    combined_df.sort_values(by=['group'])
    avg_scores = combined_df.groupby('group')['cumulative_min'].mean().reset_index()

    plt.plot(combined_df['group'].unique()[:-1], avg_scores['cumulative_min'][:-1], label='Average')

    plt.legend(fontsize='x-small')
    plt.savefig('plots/best_scores_time')
    plt.clf()


def plot_average_scores_time(filename):
    # Plots Scores vs Time graphs for average and cumulative best heuristic scores over time

    alife_data = pd.read_pickle(filename)
    i = 1

    for seed in alife_data:
        df = pd.DataFrame.from_dict(seed['result']['heuristics'])

        # Get max scores every 30s
        df.sort_values(by=['creation'])
        df['creation'] -= df['creation'][0]
        df['group'] = (df['creation'] / 30000).apply(np.floor).astype(int)
        avg_scores = list(df.groupby('group')['score'].mean())

        # plot average scores_time
        intervals = df['group'].unique()
        plt.figure(figsize=(10, 6))
        font = {'size': 16}
        plt.rc('font', **font)
        plt.rc('xtick', labelsize=12)
        plt.rc('ytick', labelsize=12)
        plt.plot(intervals, avg_scores, marker='o', label='Average Score Every 30s')
        plt.xlabel('Time (Every 30s)')
        plt.ylabel('Average Heuristic Score')
        plt.title('Average Score of Heuristics Every 30s')
        plt.legend(fontsize='x-small')
        plt.grid(True)
        plt.savefig('plots/seed' + str(i) + '_avg_scores_time')
        plt.clf()

        i += 1


def plot_all_scores(filename):
    # Plots scatter plot of all scores
    # TODO: Implement rolling window with error bars

    alife_data = pd.read_pickle(filename)
    i = 1
    for seed in alife_data:
        df = pd.DataFrame.from_dict(seed['result']['heuristics'])
        df.sort_values(by=['creation'])
        df['creation'] -= df['creation'][0]

        plt.figure(figsize=(25, 12))
        font = {'size': 16}
        plt.rc('font', **font)
        plt.rc('xtick', labelsize=12)
        plt.rc('ytick', labelsize=12)
        plt.scatter(df['creation'], df['score'], label='Heuristic Scores')
        plt.xlabel('Time')
        plt.ylabel('Score')
        plt.title('Scatter Plot of Heuristic Scores Over Time')
        plt.legend(fontsize='x-small')

        plt.grid(True)
        plt.savefig('plots/seed' + str(i) + '_scatter_plot')
        i += 1


def plot_score_path_length(filename):
    # TODO: Complete this once we get the data
    pass


def plot_histogram(filename):
    # Plot histograms for scores of heuristics

    alife_data = pd.read_pickle(filename)
    ranges = []
    for j in range(0, 2800, 200):
        ranges.append(j)

    i = 1
    for seed in alife_data:
        df = pd.DataFrame.from_dict(seed['result']['heuristics'])

        plt.figure(figsize=(15, 8))
        plt.hist(df['score'], bins=ranges, color='blue')

        plt.xlabel('Scores of Heuristics')
        plt.xticks(ranges)
        plt.ylabel('Frequency')
        plt.title('Histogram of Heuristic Scores')

        plt.grid(True)
        plt.savefig('plots/seed' + str(i) + '_heuristics_histogram')
        i += 1


plot_best_scores_time(FILENAME)
plot_average_scores_time(FILENAME)
plot_histogram(FILENAME)
plot_score_path_length(FILENAME)
plot_all_scores(FILENAME)
