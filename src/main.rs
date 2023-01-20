use type_state_builder::RequestBuilder;

mod errors;
mod prelude;
mod builder;
mod type_state_builder;


fn main(){
    // will not compile without 'url' setting
    let _request = RequestBuilder::new()
        .url("https://example.com")
        .method("GET")
        .build();
}
