use uuid::Uuid;

pub trait Crud {
    type ReadModel;
    type CreateModel;
    type UpdateModel;

    fn get_by_id(
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<Self::ReadModel, String>> + Send;
    fn create(
        data: Self::CreateModel,
    ) -> impl std::future::Future<Output = Result<Self::ReadModel, String>> + Send;
    fn update(
        data: Self::UpdateModel,
    ) -> impl std::future::Future<Output = Result<Self::ReadModel, String>> + Send;
    fn delete(id: Uuid) -> impl std::future::Future<Output = Result<Uuid, String>> + Send;
}
