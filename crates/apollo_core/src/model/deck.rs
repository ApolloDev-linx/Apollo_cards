#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    pub id: DeckId,
    pub name: String,
    }
impl Deck{
    pub fn new(id: u64, name: impl Into<String>) -> Result<Self , String>
    let name = name.into();
    if name.trim().is_empty(){
        return Err("deck name cannot be empty.".into());
    }
    Ok(Self {id: DeckId(id), name})
}
