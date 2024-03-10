pub fn from_model_to_dto_list<MODEL, DTO, F>(models: Vec<MODEL>, transform: F) -> Vec<DTO>
  where
    F: Fn(&MODEL) -> DTO,
{
  models.into_iter().map(|model| transform(&model)).collect()
}