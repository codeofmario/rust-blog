use serde::de::DeserializeOwned;

pub async fn map_body_to_model<DTO, MODEL>(
    req: DTO,
    mapper: fn(DTO) -> MODEL,
) -> MODEL
    where
        DTO: DeserializeOwned,
{
    mapper(req)
}