use actix_web::{HttpResponse, Responder, web, ResponseError};
use uuid::Uuid;
use validator::Validate;
use crate::error::*;
use crate::page::*;
use crate::repo::*;
use crate::task::{repo::*, requests::*};

pub async fn create_task(task: web::Json<CreateTaskRequest>, repo: web::Data<TaskRepository>) -> impl Responder {
    let validate = task.validate();
    if validate.is_err() {
        return ApiError::BadParam(&validate.unwrap_err().to_string()).error_response();
    }

    let result = repo.create(&task).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn delete_task(id: web::Path<Uuid>, repo: web::Data<TaskRepository>) -> impl Responder {
    let result = repo.delete(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_task(id: web::Path<Uuid>, repo: web::Data<TaskRepository>) -> impl Responder {
    let result = repo.get(&id).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn get_tasks(page: web::Query<TaskPage>, repo: web::Data<TaskRepository>) -> impl Responder {
    if page.validate().is_err() {
        return ApiError::BadPaginate(page.count).error_response();
    }

    let result = repo.paginate(&page).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}

pub async fn update_task(id: web::Path<Uuid>, task: web::Json<UpdateTaskRequest>, repo: web::Data<TaskRepository>) -> impl Responder {
    let validate = task.validate();
    if validate.is_err() {
        return ApiError::BadParam(&validate.unwrap_err().to_string()).error_response();
    }

    let result = repo.update(&id, &task).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::BadRequest().body(format!("{err:?}"))
    }
}
