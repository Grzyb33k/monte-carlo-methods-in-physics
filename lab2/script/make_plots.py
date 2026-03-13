import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

import os
from pathlib import Path

from scipy.stats import chisquare

###############################################

def f(x):
    return 4/5 * (1 + x - x**3)

SCRIPT_DIR = Path(__file__).resolve().parent

ROOT_DIR = SCRIPT_DIR.parent

data_dir = ROOT_DIR / "results"
output_dir = ROOT_DIR / "plots"

if not output_dir.exists():
    os.makedirs(output_dir)

if not data_dir.exists():
    raise FileNotFoundError(f"Data directory {data_dir} does not exist")


expected = pd.read_csv(f"{data_dir}/wyniki_expected.csv", header=None).values.flatten()
df_composite = pd.read_csv(f"{data_dir}/wyniki_complex.csv", header=None)
df_elimination = pd.read_csv(f"{data_dir}/wyniki_elimination.csv", header=None)
df_metropolis_d_0_05 = pd.read_csv(f"{data_dir}/wyniki_metropolis_d_0.05.csv", header=None)
df_metropolis_d_0_5 = pd.read_csv(f"{data_dir}/wyniki_metropolis_d_0.5.csv", header=None)

observed = {
    "composite": df_composite.values.flatten(),
    "elimination": df_elimination.values.flatten(),
    "metropolis_d_0_05": df_metropolis_d_0_05.values.flatten(),
    "metropolis_d_0_5": df_metropolis_d_0_5.values.flatten(),
}

labels = {
    "composite": "Rozkład złożony",
    "elimination": "Metoda eliminacji",
    "metropolis_d_0_05": "Metropolis $d = 0.05$",
    "metropolis_d_0_5": "Metropolis $d = 0.5$",
}


fs = 11

for key in observed:
    plt.figure(figsize=(4, 3))
    
    y = observed[key] / 1e6 / 1e-1
    y = np.append(y, 0)
    
    steps = np.linspace(0, 1, 11)
    x = np.linspace(0, 1, 100)
    
    plt.step(steps, y, where="post", label="Obserwowane")
    plt.plot(x, f(x), color="red", label="Oczekiwane")
    
    plt.text(0.2, 0.9, f"$\\chi^2 = ${chisquare(observed[key], f_exp=expected*1e6)[0]:.2f}\n$p = ${chisquare(observed[key], f_exp=expected*1e6)[1]:.3f}",
             ha='center', va='center', transform=plt.gca().transAxes, fontsize=fs-1)
    
    plt.title(labels[key], fontsize=fs)
    plt.xlabel("x", fontsize=fs-1)
    plt.ylabel("P(x)", fontsize=fs-1)
    plt.grid()
    plt.legend(fontsize=fs-2)

    plt.xlim(0, 1)
    plt.ylim(0.8, 1.15) 

    plt.tight_layout()
    
    plt.savefig(output_dir / f"{key}.pdf")
    plt.close()
