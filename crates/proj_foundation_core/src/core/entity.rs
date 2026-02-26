#[derive(Clone)]
pub struct AudioRefId(pub String);

#[derive(Clone)]
pub struct AudioRef {
    pub id: AudioRefId,
    pub path: String,
}
