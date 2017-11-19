use tputil;

pub type SpaceID = usize;

pub struct Space {
    pub id: SpaceID,
    pub transitions: Box<[SpaceTransition]>,
    pub pos: tputil::Point2D
}

pub struct SpaceTransition {
    pub to: SpaceID,
}

pub struct Board {
    pub spaces: Vec<Space>
}

impl Board {
    pub fn get_default_board() -> Board {
        return Board {
            spaces: vec![
                Space {
                    id: 54,
                    transitions: Box::new([SpaceTransition { to: 32 }]),
                    pos: tputil::Point2D::new(0.0, 0.0)
                },
                Space {
                    id: 32,
                    transitions: Box::new([SpaceTransition { to: 54 }]),
                    pos: tputil::Point2D::new(2.0, 0.7)
                },
            ]
        };
    }

    pub fn get_space(&self, id: SpaceID) -> Option<&Space> {
        for space in &self.spaces {
            if space.id == id {
                return Some(space);
            }
        }
        return None;
    }
}
