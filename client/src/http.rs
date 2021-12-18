#[async_trait]
pub trait BaseClient {
    pub async fn get(url :&str, header: HashMap<String,String>) -> Result<Vec<u8>, Error>;
    pub async fn post(url :&str, header: HashMap<String,String>) -> Result<Vec<u8>, Error>;
    pub async fn put(url :&str, header: HashMap<String,String>) -> Result<Vec<u8>, Error>;
    pub async fn patch(url :&str, header: HashMap<String,String>) -> Result<Vec<u8>, Error>;
    pub async fn delete(url :&str, header: HashMap<String,String>) -> Result<Vec<u8>, Error>;
}

