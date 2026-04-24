use crate::model::state::Flocker;
use crate::{COHESION, JUMP, MATCH, SEPARATE, SEPARATION};
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::field_2d::{toroidal_distance, toroidal_transform, Location2D};
use krabmaga::engine::location::Real2D;
use krabmaga::engine::state::State;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Bird {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
}

impl Bird {
    pub fn new(id: u32, loc: Real2D, last_d: Real2D) -> Self {
        Bird { id, loc, last_d }
    }
}

impl Agent for Bird {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<Flocker>().unwrap();
        let neighbors = state
            .field1
            .get_neighbors_within_relax_distance(self.loc, state.visual_distance);

        let width = state.dim.0;
        let height = state.dim.1;

        let mut count = 0;
        let mut cohere = Real2D { x: 0.0, y: 0.0 };
        let mut separate = Real2D { x: 0.0, y: 0.0 };
        let mut m = Real2D { x: 0.0, y: 0.0 };

        for elem in &neighbors {
            if self.id == elem.id {
                continue;
            }

            count += 1;
            let heading = Real2D {
                x: toroidal_distance(self.loc.x, elem.loc.x, width),
                y: toroidal_distance(self.loc.y, elem.loc.y, height),
            };
            cohere = Real2D {
                x: cohere.x + heading.x,
                y: cohere.y + heading.y,
            };
            if heading.x * heading.x + heading.y * heading.y < SEPARATION * SEPARATION {
                separate = Real2D {
                    x: separate.x - heading.x,
                    y: separate.y - heading.y,
                };
            }
            m = Real2D {
                x: m.x + elem.last_d.x,
                y: m.y + elem.last_d.y,
            };
        }

        let n = (count.max(1)) as f32;
        cohere = Real2D {
            x: (cohere.x / n) * COHESION,
            y: (cohere.y / n) * COHESION,
        };
        separate = Real2D {
            x: (separate.x / n) * SEPARATE,
            y: (separate.y / n) * SEPARATE,
        };
        m = Real2D {
            x: (m.x / n) * MATCH,
            y: (m.y / n) * MATCH,
        };

        let mut dx = (self.last_d.x + cohere.x + separate.x + m.x) / 2.0;
        let mut dy = (self.last_d.y + cohere.y + separate.y + m.y) / 2.0;

        let dis = (dx * dx + dy * dy).sqrt();
        if dis > 0.0 {
            dx = dx / dis * JUMP;
            dy = dy / dis * JUMP;
        }

        self.last_d = Real2D { x: dx, y: dy };

        let loc_x = toroidal_transform(self.loc.x + dx, width);
        let loc_y = toroidal_transform(self.loc.y + dy, height);

        self.loc = Real2D { x: loc_x, y: loc_y };
        state
            .field1
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }
}

impl Hash for Bird {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Bird {}

impl PartialEq for Bird {
    fn eq(&self, other: &Bird) -> bool {
        self.id == other.id
    }
}

impl Location2D<Real2D> for Bird {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Bird {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} loc {}", self.id, self.loc)
    }
}
