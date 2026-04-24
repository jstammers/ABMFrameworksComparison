import numpy as np
import polars as pl

from mesa_frames import AgentSet, Model


class FlockingAgents(AgentSet):
    def __init__(
        self,
        model,
        n_agents,
        width,
        height,
        vision,
        speed=1.0,
        separation=1.0,
        cohere=0.03,
        separate=0.015,
        match=0.05,
    ):
        super().__init__(model)
        self.width = float(width)
        self.height = float(height)
        self.vision = float(vision)
        self.speed = float(speed)
        self.separation = float(separation)
        self.cohere = float(cohere)
        self.separate = float(separate)
        self.match = float(match)

        self += pl.DataFrame(
            {
                "x": self.random.random(n_agents) * self.width,
                "y": self.random.random(n_agents) * self.height,
                "vx": self.random.uniform(-1.0, 1.0, n_agents),
                "vy": self.random.uniform(-1.0, 1.0, n_agents),
            }
        )

    def step(self):
        if len(self.df) == 0:
            return

        x = self.df["x"].to_numpy().astype(np.float64)
        y = self.df["y"].to_numpy().astype(np.float64)
        vx = self.df["vx"].to_numpy().astype(np.float64)
        vy = self.df["vy"].to_numpy().astype(np.float64)

        n = x.size
        if n == 0:
            return

        for i in self.random.permutation(n):
            dx = x - x[i]
            dy = y - y[i]
            dx -= np.rint(dx / self.width) * self.width
            dy -= np.rint(dy / self.height) * self.height

            dist2 = dx * dx + dy * dy
            in_view = dist2 <= (self.vision * self.vision)
            in_view[i] = False

            count = int(np.sum(in_view))
            n_safe = max(count, 1)

            cohere_x = np.sum(dx[in_view]) / n_safe * self.cohere
            cohere_y = np.sum(dy[in_view]) / n_safe * self.cohere

            sep_mask = in_view & (dist2 < (self.separation * self.separation))
            separate_x = -np.sum(dx[sep_mask]) / n_safe * self.separate
            separate_y = -np.sum(dy[sep_mask]) / n_safe * self.separate

            match_x = np.sum(vx[in_view]) / n_safe * self.match
            match_y = np.sum(vy[in_view]) / n_safe * self.match

            vx[i] = (vx[i] + cohere_x + separate_x + match_x) / 2.0
            vy[i] = (vy[i] + cohere_y + separate_y + match_y) / 2.0

            norm = np.hypot(vx[i], vy[i])
            if norm == 0.0:
                vx[i], vy[i] = 1.0, 0.0
            else:
                vx[i] /= norm
                vy[i] /= norm

            x[i] = (x[i] + self.speed * vx[i]) % self.width
            y[i] = (y[i] + self.speed * vy[i]) % self.height

        self.df = self.df.with_columns(
            [
                pl.Series("x", x),
                pl.Series("y", y),
                pl.Series("vx", vx),
                pl.Series("vy", vy),
            ]
        )


class FlockingModel(Model):
    def __init__(self, seed, population, width, height, vision):
        super().__init__(seed)
        self.sets += FlockingAgents(self, population, width, height, vision)

    def step(self):
        self.sets.do("step")
