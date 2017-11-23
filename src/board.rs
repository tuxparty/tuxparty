use tputil;

pub type SpaceID = usize;

#[derive(Copy, Clone)]
pub enum SpaceType {
    Positive,
    Negative
}

#[derive(Clone)]
pub struct Space {
    pub id: SpaceID,
    pub transitions: Box<[SpaceTransition]>,
    pub pos: tputil::Point2D,
    pub space_type: SpaceType
}

#[derive(Copy, Clone)]
pub struct SpaceTransition {
    pub to: SpaceID,
}

#[derive(Clone)]
pub struct Board {
    pub spaces: Vec<Space>
}

impl Board {
    pub fn get_default_board() -> Board {
        return Board {
            spaces: vec![
                Space {
                    id: 0,
                    transitions: Box::new([SpaceTransition { to: 32 }]),
                    pos: tputil::Point2D::new(0.0, 0.0),
                    space_type: SpaceType::Positive
                },
                Space {
                    id: 32,
                    transitions: Box::new([SpaceTransition { to: 66 }]),
                    pos: tputil::Point2D::new(2.0, 0.7),
                    space_type: SpaceType::Negative
                },
                Space {
                    id: 66,
                    transitions: Box::new([SpaceTransition { to: 0 }]),
                    pos: tputil::Point2D::new(-1.2, 1.2),
                    space_type: SpaceType::Positive
                }
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
