import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import os
from pathlib import Path

def plot_results(data, y_name, p_values, ylabel, name):
    colors = ['black', 'red', 'blue']

    fig, ax = plt.subplots(figsize=(10, 6))

    for i, p in enumerate(p_values):
        data = df[df['p'] == p]
        x = data['k']
        y = data[y_name]

        ax.plot(x, y, marker='o', label=f'p={p}', color=colors[i])

    # ax.set_xscale('log')
    ax.set_yscale('log')
    ax.set_xlabel('$10^k$')
    ax.set_ylabel(ylabel)
    ax.set_xlim(np.min(x), np.max(x))

    ax.grid(True, which="major", ls="--", lw=0.5)
    plt.tight_layout()
    plt.savefig(f'{output_dir}/{name}.pdf')



SCRIPT_DIR = Path(__file__).resolve().parent

ROOT_DIR = SCRIPT_DIR.parent

data_dir = ROOT_DIR / "results"
output_dir = ROOT_DIR / "plots"


df = pd.read_csv(f'{data_dir}/wyniki.csv')


if not os.path.exists(output_dir):
    os.makedirs(output_dir)

p_values = df['p'].unique()

plot_results(df, 'err_x', p_values, 'Błąd względny średniej', 'mean_error_plot')

plot_results(df, 'err_var', p_values, 'Błąd względny wariancji', 'variance_error_plot')


# colors = ['black', 'red', 'blue']

# fig, ax = plt.subplots(figsize=(10, 6))

# for i, p in enumerate(p_values):
#     data = df[df['p'] == p]
#     x = data['k']
#     y = data['err_x']

#     ax.plot(x, y, marker='o', label=f'p={p}', color=colors[i])

# # ax.set_xscale('log')
# ax.set_yscale('log')
# ax.set_xlabel('$10^k$')
# ax.set_ylabel('Błąd względny średniej')

# plt.savefig(f'{output_dir}/mean_error_plot.pdf')




# fig, ax = plt.subplots(figsize=(10, 6))

# for i, p in enumerate(p_values):
#     data = df[df['p'] == p]
#     x = data['k']
#     y = data['err_var']

#     ax.plot(x, y, marker='o', label=f'p={p}', color=colors[i])

# # ax.set_xscale('log')
# ax.set_yscale('log')
# ax.set_xlabel('$10^k$')
# ax.set_ylabel('Błąd względny wariancji')

# plt.savefig(f'{output_dir}/variance_error_plot.pdf')