use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};

use crate::models::{User, HistoryResponse};
use serde::Deserialize;
use crate::utils::{
    establish_connection,
    get_device_and_ajax,
    get_request_user_data,
    is_signed_in,
    get_first_load_page,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use sailfish::TemplateOnce;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
    config.route("/about/", web::get().to(about));
    config.route("/feedback/", web::post().to(create_feedback));
    config.route("/feedback_list/", web::get().to(feedback_list_page));
    config.route("/serve_list/", web::get().to(serve_list_page));
    config.route("/load_item/", web::get().to(get_load_page));
    config.route("/create_history/", web::get().to(create_history));
    config.route("/object_history/{id}/", web::get().to(object_history));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}

pub async fn index(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "Главная страница".to_string()).await
    }
    else {
        use crate::models::{Work, Service, Wiki, Blog, Store};

        for header in req.headers().into_iter() {
            if header.0 == "cookie" {
                let str_cookie = header.1.to_str().unwrap();
                let _cookie: Vec<&str> = str_cookie.split(";").collect();
                for c in _cookie.iter() {
                    let split_c: Vec<&str> = c.split("=").collect();
                    println!("name {:?}", split_c[0].trim());
                    println!("value {:?}", split_c[1]);
                }
            }
        };
        let _last_works = Work::get_3_works();
        let _last_services = Service::get_6_services();
        let _last_wikis = Wiki::get_3_wikis();
        let _last_blogs = Blog::get_3_blogs();
        let _last_stores = Store::get_3_stores();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            _request_user.create_superuser();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/mainpage.stpl")]
                struct Template {
                    request_user:  User,
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    title:         String,
                }
                let body = Template {
                    request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/mainpage.stpl")]
                struct Template {
                    //request_user:  User,
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    title:         String,
                }
                let body = Template {
                    //request_user:  _request_user,
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    title:         String,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:    Vec<Work>,
                    last_services: Vec<Service>,
                    last_wikis:    Vec<Wiki>,
                    last_blogs:    Vec<Blog>,
                    last_stores:   Vec<Store>,
                    is_ajax:       i32,
                    title:         String,
                }
                let body = Template {
                    last_works:    _last_works,
                    last_services: _last_services,
                    last_wikis:    _last_wikis,
                    last_blogs:    _last_blogs,
                    last_stores:   _last_stores,
                    is_ajax:       is_ajax,
                    title:         "Главная страница".to_string(),
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn about(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    // первая отрисовка страницы - организуем скрытие информации
    if is_ajax == 0 {
        get_first_load_page(&session, is_desctop, "О нас".to_string()).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/about.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      i32,
                title:        String,
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                title:        "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/about.stpl")]
            struct Template {
                //request_user: User,
                is_ajax:      i32,
                title:        String,
            }
            let body = Template {
                //request_user: _request_user,
                is_ajax:      is_ajax,
                title:        "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_about.stpl")]
            struct Template {
                is_ajax: i32,
                title:   String,
            }
            let body = Template {
                is_ajax: is_ajax,
                title:   "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/anon_about.stpl")]
            struct Template {
                is_ajax: i32,
                title:   String,
            }
            let body = Template {
                is_ajax: is_ajax,
                title:   "О нас".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use std::borrow::BorrowMut;
    use crate::models::{Feedback, NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}

pub async fn feedback_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
        if !is_signed_in(&session) {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            use crate::schema::feedbacks::dsl::feedbacks;
            use crate::models::Feedback;

            let _connection = establish_connection();
            let _feedbacks = feedbacks
                .load::<Feedback>(&_connection)
                .expect("E");

            let _request_user = get_request_user_data(&session);
            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            if _request_user.perm < 60 {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/feedback_list.stpl")]
                struct Template {
                    title:         String,
                    request_user:  User,
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    title:         "Сообщения с формы".to_string(),
                    request_user:  _request_user,
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/feedback_list.stpl")]
                struct Template {
                    title:         String,
                    //request_user:  User,
                    is_ajax:       i32,
                    feedback_list: Vec<Feedback>,
                }
                let body = Template {
                    title:         "Сообщения с формы".to_string(),
                    //request_user:  _request_user,
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
}

pub async fn serve_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let all_tech_categories: Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::position.asc())
        .load(&_connection)
        .expect("E.");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/serve_list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                title:        "Список опций и услуг".to_string(),
                request_user: _request_user,
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/serve_list.stpl")]
            struct Template {
                title:        String,
                request_user: User,
                is_ajax:      i32,
                tech_cats:    Vec<TechCategories>,
            }
            let body = Template {
                title:        "Список опций и услуг".to_string(),
                request_user: _request_user,
                is_ajax:      is_ajax,
                tech_cats:    all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_serve_list.stpl")]
            struct Template {
                title:     String,
                is_ajax:   i32,
                tech_cats: Vec<TechCategories>,
            }
            let body = Template {
                title:     "Список опций и услуг".to_string(),
                is_ajax:   is_ajax,
                tech_cats: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_serve_list.stpl")]
            struct Template {
                title:     String,
                is_ajax:   i32,
                tech_cats: Vec<TechCategories>,
            }
            let body = Template {
                title:     "Список опций и услуг".to_string(),
                is_ajax:   is_ajax,
                tech_cats: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadParams {
    pub _object_type: String,
    pub _owner_type:  String,
    pub _object_pk:   i32,
    pub _owner_pk:    i32,
}
pub async fn get_load_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::schema;

    let mut _object_type: String = "".to_string();
    let mut _owner_type: String = "".to_string();
    let mut _object_pk: i32 = 0;
    let mut _owner_pk: i32 = 0;

    let _connection = establish_connection();
    let params = web::Query::<LoadParams>::from_query(&req.query_string());
    if params.is_ok() {
        let wrap = params.unwrap();
        if wrap._object_type != "".to_string() {
            _object_type = wrap._object_type.clone();
        }
        if wrap._owner_type != "".to_string() {
            _owner_type = wrap._owner_type.clone();
        }
        if wrap._object_pk != 0 {
            _object_pk = wrap._object_pk.clone();
        }
        if wrap._owner_pk != 0 {
            _owner_pk = wrap._owner_pk.clone();
        }
    }

    if _object_type == "serve_category".to_string() {
        use crate::models::ServeCategories;
        use crate::schema::serve_categories::dsl::serve_categories;

        let _serve_categorys = serve_categories
            .filter(schema::serve_categories::id.eq(&_object_pk))
            .load::<ServeCategories>(&_connection)
            .expect("E");
        let _serve_category = _serve_categorys.into_iter().nth(0).unwrap();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve_category.stpl")]
        struct Template {
            title:        String,
            object:      ServeCategories,
            //object_type: String,
        }
        let body = Template {
            title:        "Информация о технология услуг".to_string() + &_serve_category.name,
            object:      _serve_category,
            //object_type: "serve_category".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

    } else if _object_type == "serve".to_string() && _owner_type != "service".to_string() {
        use crate::models::Serve;
        use crate::schema::serve::dsl::serve;

        let _serves = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");

        let _serve = _serves.into_iter().nth(0).unwrap();

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            title:        String,
            object:      Serve,
            //object_type: String,
        }
        let body = Template {
            title:        "Информация об опции".to_string() + &_serve.name,
            object:      _serve,
            //object_type: "serve".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if _object_type == "serve".to_string() && _owner_type == "service".to_string() {
        use crate::models::{Serve, Service};
        use crate::schema::{
            serve::dsl::serve,
            services::dsl::services
        };

        let _serves = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");
        let _serve = _serves.into_iter().nth(0).unwrap();

        let _service_id: i32 = _owner_pk;
        let _service = services
            .filter(schema::services::id.eq(&_service_id))
            .load::<Service>(&_connection)
            .expect("E");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            title:        String,
            object:      Serve,
            //object_type: String,
            //service:     Service,
        }
        let body = Template {
            title:        "Информация об опции услуги".to_string() + &_serve.name,
            object:      _serve,
            //object_type: "serve".to_string(),
            //service:     _service.into_iter().nth(0).unwrap(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryParams {
    pub id:     i32,
    pub page:   i16,
    pub link:   String,
    pub title:  String,
    pub height: f64,
    pub speed:  i16,
}

pub async fn create_history(req: HttpRequest) -> web::Json<HistoryResponse> {
    use crate::models::{CookieUser, CookieStat};

    let params = web::Query::<HistoryParams>::from_query(&req.query_string());
    let params_2 = params.unwrap();
    let p_id = params_2.id;

    let current_id = CookieUser::get_user(p_id, &req).id;
    let p_page = params_2.page;
    let p_link = params_2.link.clone();
    let p_title = params_2.title.clone();
    let p_height = params_2.height;
    let p_speed = params_2.speed;
    return CookieStat::create (
        current_id,
        p_page,
        p_link,
        p_title,
        p_height,
        p_speed
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
}
pub async fn object_history(req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    use crate::models::CookieUser;

    let _user = CookieUser::get_user(*id, &req);
    return web::Json( ObjectResponse {
        id:         _user.id,
        ip:         _user.ip,
        device:     _user.device,
        city_ru:    _user.city_ru,
        city_en:    _user.city_en,
        region_ru:  _user.region_ru,
        region_en:  _user.region_en,
        country_ru: _user.country_ru,
        country_en: _user.country_en,
    })
}
