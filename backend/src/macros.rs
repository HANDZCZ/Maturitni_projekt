macro_rules! build_resp_inner {
    ($name:ident ,$path:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                actix_web::HttpResponse::$path().finish()
            };
            ($message:expr) => {
                actix_web::HttpResponse::$path().body($message)
            };
        }
    };
}
macro_rules! build_resp {
    ($name:ident ,$path:ident) => {
        build_resp_inner!($name, $path);
    };
    ($name:ident ,$path:ident, $json_name:ident) => {
        build_resp_inner!($name, $path);
        #[macro_export]
        macro_rules! $json_name {
            ($message:expr) => {
                actix_web::HttpResponse::$path().json2($message)
            };
        }
    };
}

build_resp!(resp_500_IntSerErr, InternalServerError);
build_resp!(resp_400_BadReq, BadRequest);
build_resp!(resp_200_Ok, Ok, resp_200_Ok_json);
build_resp!(resp_401_Unauth, Unauthorized);

/*#[macro_export]
macro_rules! resp_500_IntSerErr {
    () => {
        actix_web::HttpResponse::InternalServerError().finish()
    };
    ($message:expr) => {
        actix_web::HttpResponse::InternalServerError().body($message)
    };
}

#[macro_export]
macro_rules! resp_400_BadReq {
    () => {
        actix_web::HttpResponse::BadRequest().finish()
    };
    ($message:expr) => {
        actix_web::HttpResponse::BadRequest().body($message)
    };
}

#[macro_export]
macro_rules! resp_200_Ok {
    () => {
        actix_web::HttpResponse::Ok().finish()
    };
    ($message:expr) => {
        actix_web::HttpResponse::Ok().body($message)
    };
}*/

#[cfg(feature = "time_it_macro")]
#[macro_export]
macro_rules! time_it {
    ($message:expr, $what:block) => {{
        use colored::*;
        let time_it_start_time = std::time::SystemTime::now();
        let time_it_measure_start = std::time::Instant::now();
        let time_it_res = $what;
        let time_it_measured = time_it_measure_start.elapsed();

        println!(
            "{opening_bracket}{start_time} {mark} {module_path}{clocing_bracket} {message} {arrow} {time:?}",
            opening_bracket = "[".bright_black(),
            start_time = humantime::format_rfc3339_seconds(time_it_start_time),
            mark = "Time it".bright_blue(),
            clocing_bracket = "]".bright_black(),
            module_path = module_path!(),
            message = $message,
            arrow = "=>".green(),
            time = time_it_measured,
        );
        time_it_res
    }};
}

#[cfg(not(feature = "time_it_macro"))]
#[macro_export]
macro_rules! time_it {
    ($message:expr, $what:block) => {
        $what
    };
}

macro_rules! build_actix_err {
    ($name:ident ,$path:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                $name!("")
            };
            ($message:expr) => {
                actix_web::error::$path($message)
            };
        }
    };
}

build_actix_err!(actix_err_500_IntSerErr, ErrorInternalServerError);
