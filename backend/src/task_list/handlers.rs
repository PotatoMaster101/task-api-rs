use actix_web::{HttpResponse, Responder, ResponseError, web};
use uuid::Uuid;
use validator::Validate;
use crate::error::*;
use crate::page::*;
use crate::repo::*;
use crate::task_list::{repo::*, requests::*};

pub async fn create_task_list(list: web::Json<CreateTaskListRequest>, repo: web::Data<TaskListRepository>) -> impl Responder {
    let validate = list.validate();
    if validate.is_err() {
        return ApiError::BadParam(&validate.unwrap_err().to_string()).error_response();
    }

    let result = repo.create(&list).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn delete_task_list(id: web::Path<Uuid>, repo: web::Data<TaskListRepository>) -> impl Responder {
    let result = repo.delete(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_task_list(id: web::Path<Uuid>, repo: web::Data<TaskListRepository>) -> impl Responder {
    let result = repo.get(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_task_lists(page: web::Query<Page>, repo: web::Data<TaskListRepository>) -> impl Responder {
    if page.validate().is_err() {
        return ApiError::BadPaginate(page.count).error_response();
    }

    let result = repo.paginate(&page).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn update_task_list(id: web::Path<Uuid>, list: web::Json<UpdateTaskListRequest>, repo: web::Data<TaskListRepository>) -> impl Responder {
    let validate = list.validate();
    if validate.is_err() {
        return ApiError::BadParam(&validate.unwrap_err().to_string()).error_response();
    }

    let result = repo.update(&id, &list).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}
