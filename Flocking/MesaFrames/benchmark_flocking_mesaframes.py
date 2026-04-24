import gc
import timeit

setup = """
gc.enable()
import random
random.seed(42)

from FlockingMesaFrames import FlockingModel

def runthemodel(seed, population, width, height, vision):
    model = FlockingModel(seed, population, width, height, vision)
    for _ in range(100):
        model.step()

seed = random.randint(1, 10000)
population = {}
width = {}
height = {}
vision = {}
"""

n_run = 100

tt = timeit.Timer(
    "runthemodel(seed, population, width, height, vision)",
    setup=setup.format(200, 100, 100, 5),
)
a = tt.repeat(n_run, 1)
median_time = sorted(a)[n_run // 2 + n_run % 2]
print("Mesa-Frames Flocking-small (ms):", median_time * 1e3)

tt = timeit.Timer(
    "runthemodel(seed, population, width, height, vision)",
    setup=setup.format(400, 150, 150, 15),
)
a = tt.repeat(n_run, 1)
median_time = sorted(a)[n_run // 2 + n_run % 2]
print("Mesa-Frames Flocking-large (ms):", median_time * 1e3)
