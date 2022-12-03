use aide::{
    axum::{
        routing::{post_with, put_with},
        ApiRouter, IntoApiResponse,
    },
    transform::TransformOperation,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set};

use entity::post;

pub mod docs;
mod query;
mod responses;

pub fn post_router() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create, create_docs).get_with(list, list_docs),
        )
        .api_route(
            "/:id",
            put_with(update, update_docs).delete_with(delete, delete_docs),
        )
}

pub async fn create(
    Extension(db): Extension<&DatabaseConnection>,
    Json(request): Json<post::Model>,
) -> impl IntoApiResponse {
    post::ActiveModel {
        title: Set(request.title.into()),
        text: Set(request.text.into()),
        is_checked: Set(request.is_checked.into()),
        name: Set(request.name.into()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("insert fail");
    (StatusCode::CREATED, "insert ok")
}

pub fn create_docs(op: TransformOperation) -> TransformOperation {
    op.summary("创建文章")
        .response_with::<201, (), _>(|res| res.description("创建成功"))
}

pub async fn update(
    Extension(db): Extension<&DatabaseConnection>,
    Path(id): Path<i32>,
    Json(request): Json<post::Model>,
) -> impl IntoApiResponse {
    let post: post::ActiveModel = post::Entity::find_by_id(id)
        .one(db)
        .await
        .expect("cannot found")
        .map(Into::into)
        .expect("change error");

    post::ActiveModel {
        id: post.id,
        title: Set(request.title.to_owned()),
        text: Set(request.text.into()),
        name: Set(request.name.to_owned()),
        is_checked: Set(request.is_checked.to_owned()),
    }
    .update(db)
    .await
    .expect("update Error");
    (StatusCode::OK, "update ok")
}

pub fn update_docs(op: TransformOperation) -> TransformOperation {
    op.summary("更新文章")
        .response_with::<200, (), _>(|res| res.description("修改成功"))
}

pub async fn list(
    Extension(db): Extension<&DatabaseConnection>, // Query(params): Query<query::Params>
) -> impl IntoApiResponse {
    let params = query::Params {
        page_size: 10,
        page_num: 1,
    };
    let paginator = post::Entity::find()
        .order_by_asc(post::Column::Id)
        .paginate(db, params.page_size);
    let max_page = paginator.num_pages().await.unwrap_or(0);

    let results = paginator
        .fetch_page(params.page_num - 1)
        .await
        .expect("error fetch pages");
    Json(responses::ListResponse { max_page, results })
}

pub fn list_docs(op: TransformOperation) -> TransformOperation {
    op.summary("查询文章")
        .response::<200, Json<responses::ListResponse>>()
}

pub async fn delete(
    Extension(db): Extension<&DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoApiResponse {
    post::Entity::delete_by_id(id)
        .exec(db)
        .await
        .expect("delete error");
    StatusCode::NO_CONTENT
}

pub fn delete_docs(op: TransformOperation) -> TransformOperation {
    op.summary("删除文章")
        .response_with::<204, (), _>(|res| res.description("删除成功"))
}
