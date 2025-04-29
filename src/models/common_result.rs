use salvo::oapi;
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct CommonResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> CommonResult<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data,
        }
    }

    pub fn err(code: i32, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    pub fn empty_ok() -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            data: None,
        }
    }
}

#[async_trait]
impl<T> Writer for CommonResult<T>
where
    T: Serialize + Send + 'static,
{
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.render(Json(self));
    }
}
impl<T> oapi::ToResponse for CommonResult<T> {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::Response> {
        let response = oapi::Response::new("CommonResult response returns CommonResult entity")
            .add_content(
                "application/json",
                oapi::Content::new(
                    oapi::Object::new()
                        .property(
                            "code",
                            oapi::Object::new()
                                .schema_type(oapi::schema::SchemaType::basic(
                                    oapi::schema::BasicType::Integer,
                                ))
                                .format(oapi::SchemaFormat::KnownFormat(oapi::KnownFormat::Int32))
                                .example(0),
                        )
                        .required("code")
                        .property(
                            "msg",
                            oapi::Object::new()
                                .schema_type(oapi::schema::SchemaType::basic(
                                    oapi::schema::BasicType::Integer,
                                ))
                                .format(oapi::SchemaFormat::KnownFormat(oapi::KnownFormat::String))
                                .example("success"),
                        )
                        .required("msg")
                        .property(
                            "data",
                            oapi::Object::new().schema_type(oapi::schema::SchemaType::any()),
                        ),
                ),
            );
        components.responses.insert("CommonResult", response);
        oapi::RefOr::Ref(oapi::Ref::new(format!(
            "#/components/responses/{}",
            "CommonResult"
        )))
    }
}
impl<T> EndpointOutRegister for CommonResult<T> {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert("200", <Self as oapi::ToResponse>::to_response(components))
    }
}
