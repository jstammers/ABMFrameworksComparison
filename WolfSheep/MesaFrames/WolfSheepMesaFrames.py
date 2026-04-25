import numpy as np
import polars as pl

from mesa_frames import AgentSet, Model


def _random_unique_ids(rng, existing_ids, count):
    if count == 0:
        return np.array([], dtype=np.uint64)

    used = set(int(v) for v in existing_ids.tolist())
    out = np.empty(count, dtype=np.uint64)
    filled = 0
    high = np.iinfo(np.uint64).max
    while filled < count:
        candidate = int(rng.integers(0, high, dtype=np.uint64))
        if candidate in used:
            continue
        used.add(candidate)
        out[filled] = np.uint64(candidate)
        filled += 1
    return out


class SheepAgents(AgentSet):
    def __init__(
        self, model, initial_sheep, width, height, sheep_reproduce, sheep_gain
    ):
        super().__init__(model)
        self.width = int(width)
        self.height = int(height)
        self.sheep_reproduce = float(sheep_reproduce)
        self.sheep_gain = float(sheep_gain)
        self += pl.DataFrame(
            {
                "x": self.random.integers(0, self.width, initial_sheep),
                "y": self.random.integers(0, self.height, initial_sheep),
                "energy": self.random.random(initial_sheep) * 2.0 * self.sheep_gain,
            }
        )

    def _move(self, x, y):
        n = len(x)
        nx = np.empty(n, dtype=np.int64)
        ny = np.empty(n, dtype=np.int64)
        offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        for i in range(n):
            cx = int(x[i])
            cy = int(y[i])
            choices = []
            for dx, dy in offsets:
                tx = cx + dx
                ty = cy + dy
                if 0 <= tx < self.width and 0 <= ty < self.height:
                    choices.append((tx, ty))
            tx, ty = choices[self.random.integers(0, len(choices))]
            nx[i] = tx
            ny[i] = ty
        return nx, ny

    def step(self):
        if len(self.df) == 0:
            return

        uid = self.df["unique_id"].to_numpy().astype(np.uint64)
        x = self.df["x"].to_numpy().astype(np.int64)
        y = self.df["y"].to_numpy().astype(np.int64)
        energy = self.df["energy"].to_numpy().astype(np.float64)

        x, y = self._move(x, y)
        energy -= 1.0

        cell_id = x * self.height + y
        grown = self.model.grass[x, y] >= self.model.grass_regrowth_time
        candidate = np.where(grown)[0]
        if candidate.size > 0:
            c_cells = cell_id[candidate]
            key = self.random.random(candidate.size)
            order = np.lexsort((key, c_cells))
            ordered_cells = c_cells[order]
            ordered_idx = candidate[order]
            keep = np.ones(ordered_cells.size, dtype=bool)
            keep[1:] = ordered_cells[1:] != ordered_cells[:-1]
            eaters = ordered_idx[keep]
            eaten_cells = ordered_cells[keep]
            energy[eaters] += self.sheep_gain
            gx = eaten_cells // self.height
            gy = eaten_cells % self.height
            self.model.grass[gx, gy] = 0

        alive = energy >= 1.0
        uid = uid[alive]
        x = x[alive]
        y = y[alive]
        energy = energy[alive]

        repro = self.random.random(len(uid)) < self.sheep_reproduce
        if np.any(repro):
            energy[repro] /= 2.0
            bx = x[repro].copy()
            by = y[repro].copy()
            be = energy[repro].copy()

            new_ids = _random_unique_ids(self.random, uid, len(bx))

            uid = np.concatenate([uid, new_ids])
            x = np.concatenate([x, bx])
            y = np.concatenate([y, by])
            energy = np.concatenate([energy, be])

        self.df = pl.DataFrame(
            {
                "unique_id": pl.Series("unique_id", uid, dtype=pl.UInt64),
                "x": x,
                "y": y,
                "energy": energy,
            }
        )


class WolfAgents(AgentSet):
    def __init__(self, model, initial_wolves, width, height, wolf_reproduce, wolf_gain):
        super().__init__(model)
        self.width = int(width)
        self.height = int(height)
        self.wolf_reproduce = float(wolf_reproduce)
        self.wolf_gain = float(wolf_gain)
        self += pl.DataFrame(
            {
                "x": self.random.integers(0, self.width, initial_wolves),
                "y": self.random.integers(0, self.height, initial_wolves),
                "energy": self.random.random(initial_wolves) * 2.0 * self.wolf_gain,
            }
        )

    def _move(self, x, y):
        n = len(x)
        nx = np.empty(n, dtype=np.int64)
        ny = np.empty(n, dtype=np.int64)
        offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        for i in range(n):
            cx = int(x[i])
            cy = int(y[i])
            choices = []
            for dx, dy in offsets:
                tx = cx + dx
                ty = cy + dy
                if 0 <= tx < self.width and 0 <= ty < self.height:
                    choices.append((tx, ty))
            tx, ty = choices[self.random.integers(0, len(choices))]
            nx[i] = tx
            ny[i] = ty
        return nx, ny

    def step(self):
        if len(self.df) == 0:
            return

        uid = self.df["unique_id"].to_numpy().astype(np.uint64)
        x = self.df["x"].to_numpy().astype(np.int64)
        y = self.df["y"].to_numpy().astype(np.int64)
        energy = self.df["energy"].to_numpy().astype(np.float64)

        x, y = self._move(x, y)
        energy -= 1.0

        sheep_df = self.model.sheep_set.df
        if len(sheep_df) > 0:
            suid = sheep_df["unique_id"].to_numpy().astype(np.uint64)
            sx = sheep_df["x"].to_numpy().astype(np.int64)
            sy = sheep_df["y"].to_numpy().astype(np.int64)
            scell = sx * self.height + sy

            wcell = x * self.height + y
            w_order = np.argsort(wcell)
            s_order = np.argsort(scell)

            wcell_s = wcell[w_order]
            scell_s = scell[s_order]

            w_unique, w_start, w_count = np.unique(
                wcell_s, return_index=True, return_counts=True
            )
            s_unique, s_start, s_count = np.unique(
                scell_s, return_index=True, return_counts=True
            )
            s_map = {int(cell): i for i, cell in enumerate(s_unique)}

            eaten_mask = np.zeros(len(suid), dtype=bool)
            for wi, cell in enumerate(w_unique):
                si = s_map.get(int(cell))
                if si is None:
                    continue
                wolves_here = w_order[w_start[wi] : w_start[wi] + w_count[wi]]
                sheep_here_sorted = s_order[s_start[si] : s_start[si] + s_count[si]]

                k = min(len(wolves_here), len(sheep_here_sorted))
                if k == 0:
                    continue

                if len(wolves_here) > k:
                    wolves_here = wolves_here[
                        self.random.choice(len(wolves_here), size=k, replace=False)
                    ]
                if len(sheep_here_sorted) > k:
                    sheep_here_sorted = sheep_here_sorted[
                        self.random.choice(
                            len(sheep_here_sorted), size=k, replace=False
                        )
                    ]

                energy[wolves_here] += self.wolf_gain
                eaten_mask[sheep_here_sorted] = True

            if np.any(eaten_mask):
                self.model.sheep_set.df = pl.DataFrame(
                    {
                        "unique_id": pl.Series(
                            "unique_id", suid[~eaten_mask], dtype=pl.UInt64
                        ),
                        "x": sx[~eaten_mask],
                        "y": sy[~eaten_mask],
                        "energy": sheep_df["energy"]
                        .to_numpy()
                        .astype(np.float64)[~eaten_mask],
                    }
                )

        alive = energy >= 0.0
        uid = uid[alive]
        x = x[alive]
        y = y[alive]
        energy = energy[alive]

        repro = self.random.random(len(uid)) < self.wolf_reproduce
        if np.any(repro):
            energy[repro] /= 2.0
            bx = x[repro].copy()
            by = y[repro].copy()
            be = energy[repro].copy()

            new_ids = _random_unique_ids(self.random, uid, len(bx))

            uid = np.concatenate([uid, new_ids])
            x = np.concatenate([x, bx])
            y = np.concatenate([y, by])
            energy = np.concatenate([energy, be])

        self.df = pl.DataFrame(
            {
                "unique_id": pl.Series("unique_id", uid, dtype=pl.UInt64),
                "x": x,
                "y": y,
                "energy": energy,
            }
        )


class WolfSheepModel(Model):
    def __init__(
        self,
        seed,
        height,
        width,
        initial_sheep,
        initial_wolves,
        sheep_reproduce,
        wolf_reproduce,
        grass_regrowth_time,
        wolf_gain_from_food=13,
        sheep_gain_from_food=5,
    ):
        super().__init__(seed)
        self.height = int(height)
        self.width = int(width)
        self.grass_regrowth_time = int(grass_regrowth_time)
        self.wolf_gain_from_food = float(wolf_gain_from_food)
        self.sheep_gain_from_food = float(sheep_gain_from_food)

        self.grass = np.where(
            self.random.random((self.width, self.height)) < 0.5,
            self.grass_regrowth_time,
            self.random.integers(
                0, self.grass_regrowth_time + 1, (self.width, self.height)
            ),
        ).astype(np.int64)

        self.sheep_set = SheepAgents(
            self,
            int(initial_sheep),
            self.width,
            self.height,
            float(sheep_reproduce),
            self.sheep_gain_from_food,
        )
        self.wolf_set = WolfAgents(
            self,
            int(initial_wolves),
            self.width,
            self.height,
            float(wolf_reproduce),
            self.wolf_gain_from_food,
        )

        self.sets += self.sheep_set
        self.sets += self.wolf_set

    def step(self):
        self.sheep_set.step()
        self.wolf_set.step()
        self.grass = np.minimum(self.grass + 1, self.grass_regrowth_time)
