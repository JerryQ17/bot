#[macro_export]
macro_rules! http_get_response {
    ($self:ident, $endpoint:expr $(, $param:expr)*) => {
        {
            let mut builder = $self.get_builder($endpoint);
            $(
            builder = builder.query(&[(stringify!($param), $param)]);
            )*
            builder.send().await?
        }
    };
}

#[macro_export]
macro_rules! http_post_response {
    ($self:ident, $endpoint:expr $(, $param:expr)*) => {
        {
            let mut builder = $self.post_builder($endpoint);
            $(
            builder = builder.query(&[(stringify!($param), $param)]);
            )*
            builder.send().await?
        }
    };
}
