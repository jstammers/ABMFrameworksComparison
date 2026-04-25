import numpy as np
import polars as pl

from mesa_frames import AgentSet, Model


class SchellingAgents(AgentSet):
    def __init__(self, model, width, height, num_agents, homophily, radius):
        super().__init__(model)
        self.width = int(width)
        self.height = int(height)
        self.homophily = int(homophily)
        self.radius = int(radius)

        if num_agents > self.width * self.height:
            raise ValueError("num_agents exceeds available cells")

        all_cells = [(x, y) for x in range(self.width) for y in range(self.height)]
        chosen = self.random.choice(len(all_cells), size=num_agents, replace=False)
        positions = [all_cells[i] for i in chosen]
        groups = np.array([1 if i < num_agents / 2 else 2 for i in range(num_agents)])

        self += pl.DataFrame(
            {
                "group": groups,
                "x": np.array([p[0] for p in positions], dtype=np.int64),
                "y": np.array([p[1] for p in positions], dtype=np.int64),
                "happy": np.zeros(num_agents, dtype=bool),
            }
        )

    def step(self):
        if len(self.df) == 0:
            return

        uid = self.df["unique_id"].to_numpy()
        x = self.df["x"].to_numpy().astype(np.int64)
        y = self.df["y"].to_numpy().astype(np.int64)
        group = self.df["group"].to_numpy().astype(np.int64)
        happy = self.df["happy"].to_numpy().astype(bool)
        n = len(x)

        grid = -np.ones((self.width, self.height), dtype=np.int64)
        for i in range(n):
            grid[x[i], y[i]] = i

        empty = {(ix, iy) for ix in range(self.width) for iy in range(self.height)}
        for i in range(n):
            empty.discard((int(x[i]), int(y[i])))

        order = self.random.permutation(n)
        for i in order:
            cx = int(x[i])
            cy = int(y[i])
            same = 0

            for nx in range(
                max(0, cx - self.radius), min(self.width, cx + self.radius + 1)
            ):
                for ny in range(
                    max(0, cy - self.radius), min(self.height, cy + self.radius + 1)
                ):
                    if nx == cx and ny == cy:
                        continue
                    if max(abs(nx - cx), abs(ny - cy)) > self.radius:
                        continue
                    j = int(grid[nx, ny])
                    if j >= 0 and group[j] == group[i]:
                        same += 1

            is_happy = same >= self.homophily
            if not is_happy and empty:
                ex, ey = list(empty)[self.random.integers(0, len(empty))]
                empty.remove((ex, ey))
                empty.add((cx, cy))
                grid[cx, cy] = -1
                grid[ex, ey] = i
                x[i] = ex
                y[i] = ey

            happy[i] = is_happy

        self.df = pl.DataFrame(
            {
                "unique_id": pl.Series("unique_id", uid, dtype=pl.UInt64),
                "group": group,
                "x": x,
                "y": y,
                "happy": happy,
            }
        )


class SchellingModel(Model):
    def __init__(self, seed, height, width, homophily, radius, density):
        super().__init__(seed)
        num_agents = int(round(width * height * density))
        self.sets += SchellingAgents(self, width, height, num_agents, homophily, radius)

    def step(self):
        self.sets.do("step")
