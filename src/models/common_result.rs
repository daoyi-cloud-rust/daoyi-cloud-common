use salvo::oapi;
use salvo::prelude::*;
use serde::Serialize;
/// 通用返回结果
#[derive(Serialize, Clone, Debug)]
pub struct CommonResult<T> {
    /// 返回码
    pub code: i32,
    /// 返回信息
    pub msg: String,
    /// 返回数据
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
impl<T> oapi::ToResponse for CommonResult<T>
where
    T: oapi::ToSchema,
{
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::Response> {
        let schema_ref = T::to_schema(components);
        let response = oapi::Response::new("CommonResult response returns CommonResult entity")
            .add_content(
                "application/json",
                oapi::Content::new(
                    oapi::Object::new()
                        .property(
                            "code",
                            oapi::Object::new()
                                .description("返回码")
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
                                .description("返回信息")
                                .schema_type(oapi::schema::SchemaType::basic(
                                    oapi::schema::BasicType::String,
                                ))
                                .format(oapi::SchemaFormat::KnownFormat(oapi::KnownFormat::String))
                                .example("success"),
                        )
                        .required("msg")
                        .property("data", schema_ref),
                ),
            );
        components.responses.insert("CommonResult", response);
        oapi::RefOr::Ref(oapi::Ref::new(format!(
            "#/components/responses/{}",
            "CommonResult"
        )))
    }
}
impl<T> EndpointOutRegister for CommonResult<T>
where
    T: oapi::ToSchema,
{
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert("200", <Self as oapi::ToResponse>::to_response(components))
    }
}
