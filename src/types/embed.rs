#[derive(Serialize, Deserialize, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Embed {
    pub r#type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<time::PrimitiveDateTime>,
    pub author: Option<EmbedAuthor>,
    pub footer: Option<EmbedFooter>,
    pub fields: Vec<EmbedField>,
}
