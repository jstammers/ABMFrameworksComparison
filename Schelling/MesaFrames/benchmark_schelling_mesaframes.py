import gc
import timeit

setup = """
gc.enable()
import random
random.seed(42)

from SchellingMesaFrames import SchellingModel

def runthemodel(seed, height, width, homophily, radius, density):
    model = SchellingModel(seed, height, width, homophily, radius, density)
    for _ in range(20):
        model.step()

seed = random.randint(1, 10000)
height = {}
width = {}
homophily = {}
radius = {}
density = {}
"""

n_run = 100

tt = timeit.Timer(
    "runthemodel(seed, height, width, homophily, radius, density)",
    setup=setup.format(40, 40, 3, 1, 0.625),
)
a = tt.repeat(n_run, 1)
median_time = sorted(a)[n_run // 2 + n_run % 2]
print("Mesa-Frames Schelling-small (ms):", median_time * 1e3)

tt = timeit.Timer(
    "runthemodel(seed, height, width, homophily, radius, density)",
    setup=setup.format(100, 100, 8, 2, 0.8),
)
a = tt.repeat(n_run, 1)
median_time = sorted(a)[n_run // 2 + n_run % 2]
print("Mesa-Frames Schelling-large (ms):", median_time * 1e3)
