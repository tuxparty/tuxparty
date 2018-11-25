use tputil;

pub type SpaceID = usize;

#[derive(Copy, Clone)]
pub enum SpaceType {
    Positive,
    Negative,
}

#[derive(Clone)]
pub struct Space {
    pub id: SpaceID,
    pub transitions: Box<[SpaceTransition]>,
    pub pos: tputil::Point2D,
    pub space_type: SpaceType,
}

#[derive(Copy, Clone)]
pub struct SpaceTransition {
    pub to: SpaceID,
}

#[derive(Clone)]
pub struct Board {
    pub spaces: Vec<Space>,
}

impl Board {
    pub fn get_default_board() -> Board {
        Board {
            spaces: vec![
                Space {
                    id: 0,
                    pos: tputil::Point2D::new(20.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 1 }]),
                },
                Space {
                    id: 1,
                    pos: tputil::Point2D::new(16.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 2 }]),
                },
                Space {
                    id: 2,
                    pos: tputil::Point2D::new(12.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 3 }, SpaceTransition { to: 13 }]),
                },
                Space {
                    id: 3,
                    pos: tputil::Point2D::new(9.0, 20.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 4 }]),
                },
                Space {
                    id: 4,
                    pos: tputil::Point2D::new(5.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 5 }]),
                },
                Space {
                    id: 5,
                    pos: tputil::Point2D::new(1.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 6 }]),
                },
                Space {
                    id: 6,
                    pos: tputil::Point2D::new(-3.0, 20.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 7 }]),
                },
                Space {
                    id: 7,
                    pos: tputil::Point2D::new(-3.0, 16.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 8 }]),
                },
                Space {
                    id: 8,
                    pos: tputil::Point2D::new(-3.0, 12.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 9 }]),
                },
                Space {
                    id: 9,
                    pos: tputil::Point2D::new(-3.0, 8.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 10 }]),
                },
                Space {
                    id: 10,
                    pos: tputil::Point2D::new(-3.0, 4.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 11 }]),
                },
                Space {
                    id: 11,
                    pos: tputil::Point2D::new(-3.0, 0.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 12 }]),
                },
                Space {
                    id: 12,
                    pos: tputil::Point2D::new(0.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 18 }]),
                },
                Space {
                    id: 13,
                    pos: tputil::Point2D::new(9.0, 17.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 14 }]),
                },
                Space {
                    id: 14,
                    pos: tputil::Point2D::new(6.0, 14.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 15 }]),
                },
                Space {
                    id: 15,
                    pos: tputil::Point2D::new(3.0, 11.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 16 }]),
                },
                Space {
                    id: 16,
                    pos: tputil::Point2D::new(3.0, 7.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 17 }]),
                },
                Space {
                    id: 17,
                    pos: tputil::Point2D::new(3.0, 3.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 18 }]),
                },
                Space {
                    id: 18,
                    pos: tputil::Point2D::new(3.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 19 }]),
                },
                Space {
                    id: 19,
                    pos: tputil::Point2D::new(7.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 20 }]),
                },
                Space {
                    id: 20,
                    pos: tputil::Point2D::new(11.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 21 }, SpaceTransition { to: 26 }]),
                },
                Space {
                    id: 21,
                    pos: tputil::Point2D::new(15.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 22 }]),
                },
                Space {
                    id: 22,
                    pos: tputil::Point2D::new(19.0, 0.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 23 }]),
                },
                Space {
                    id: 23,
                    pos: tputil::Point2D::new(22.0, 0.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 24 }]),
                },
                Space {
                    id: 24,
                    pos: tputil::Point2D::new(22.0, 4.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 25 }]),
                },
                Space {
                    id: 25,
                    pos: tputil::Point2D::new(22.0, 8.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 28 }]),
                },
                Space {
                    id: 26,
                    pos: tputil::Point2D::new(15.0, 4.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 27 }]),
                },
                Space {
                    id: 27,
                    pos: tputil::Point2D::new(18.0, 7.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 25 }]),
                },
                Space {
                    id: 28,
                    pos: tputil::Point2D::new(22.0, 11.0),
                    space_type: SpaceType::Negative,
                    transitions: Box::new([SpaceTransition { to: 29 }]),
                },
                Space {
                    id: 29,
                    pos: tputil::Point2D::new(22.0, 15.0),
                    space_type: SpaceType::Positive,
                    transitions: Box::new([SpaceTransition { to: 0 }]),
                },
            ],
        }
    }

    pub fn get_space(&self, id: SpaceID) -> Option<&Space> {
        for space in &self.spaces {
            if space.id == id {
                return Some(space);
            }
        }
        None
    }
}
