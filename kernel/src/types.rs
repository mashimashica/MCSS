use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Agent,
    Spot,
    AgentSet,
    SpotSet,
    Custom(String),
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityType::Agent => write!(f, "Agent"),
            EntityType::Spot => write!(f, "Spot"),
            EntityType::AgentSet => write!(f, "Agent Set"),
            EntityType::SpotSet => write!(f, "Spot Set"),
            EntityType::Custom(s) => write!(f, "Custom({})", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

impl fmt::Display for RelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationType::OneToOne => write!(f, "OneToOne"),
            RelationType::OneToMany => write!(f, "OneToMany"),
            RelationType::ManyToOne => write!(f, "ManyToOne"),
            RelationType::ManyToMany => write!(f, "ManyToMany"),
        }
    }
}