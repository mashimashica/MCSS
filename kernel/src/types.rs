use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Person,
    Household,
    Organization,
    Location,
    Custom(String),
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityType::Person => write!(f, "Person"),
            EntityType::Household => write!(f, "Household"),
            EntityType::Organization => write!(f, "Organization"),
            EntityType::Location => write!(f, "Location"),
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