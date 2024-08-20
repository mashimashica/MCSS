#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Person,
    Household,
    Organization,
    Location,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationType {
    OneOnOne, // 1:1 関係
    Include,  // 1:N 関係
    Exist,    // N:1 関係
    Custom(String),
}