#[derive(Clone)]
pub struct AudioRefIdDto(pub String);

#[derive(Clone)]
pub struct AudioRefDto {
    pub id: AudioRefIdDto,
    pub path: String,
}
