#[derive(Serialize, Deserialize)]
pub struct InternalServerErrorJson {
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestJson {
    pub reason: String,
    pub location: Option<BadRequestJsonLocation>,
}

#[derive(Serialize, Deserialize)]
pub struct BadRequestJsonLocation {
    pub line: u32,
    pub character: u32,
}
