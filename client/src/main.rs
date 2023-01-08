//use reqwest;
use http_req::request;

fn main() {
    // let f = async {
    //     let client = reqwest::Client::new();
    //     let res = client.get("http://127.0.0.1:8080/get-id").send().await.unwrap();
    //
    //     println!("Success! {:?}", res)
    // };

    let mut writer = Vec::new(); //container for body of a response
    let res = request::get("http://127.0.0.1:8080/get-id", &mut writer).unwrap();
    let str = std::str::from_utf8(&writer).unwrap();

    println!("{res:?} \n {str:?}");
}