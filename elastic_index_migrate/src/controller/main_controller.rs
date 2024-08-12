use crate::common::*;

use crate::service::es_service::*;
use crate::service::re_index::*;

/*

*/
pub async fn main_controller() {

    info!("elastic_index_migrate start");

    // Select compilation environment
    dotenv().ok();

    let from_es_host: Vec<String> = env::var("FROM_ES_DB_URL").expect("[ENV file read Error] 'ES_DB_URL' must be set").split(',').map(|s| s.to_string()).collect();
    let from_es_id = env::var("FROM_ES_ID").expect("[ENV file read Error] 'ES_ID' must be set");
    let from_es_pw = env::var("FROM_ES_PW").expect("[ENV file read Error] s'ES_PW' must be set");

    let to_es_host: Vec<String> = env::var("TO_ES_DB_URL").expect("[ENV file read Error] 'ES_DB_URL' must be set").split(',').map(|s| s.to_string()).collect();
    let to_es_id = env::var("TO_ES_ID").expect("[ENV file read Error] 'ES_ID' must be set");
    let to_es_pw = env::var("TO_ES_PW").expect("[ENV file read Error] s'ES_PW' must be set");

    
    // Elasticsearch connection
    // 1. 
    let from_es_client: EsHelper = match EsHelper::new(from_es_host, &from_es_id, &from_es_pw) {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    };
    
    // 2.
    let to_es_client: EsHelper = match EsHelper::new(to_es_host, &to_es_id, &to_es_pw) {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    };
    
    let index_name = "consuming_index_prod_new_copy";
    let new_index_name = "consuming_index_prod_new_copy_2";
    
    match re_index_start(&from_es_client, &to_es_client, index_name, new_index_name).await {
        Ok(_) => (),
        Err(err) => {
            panic!("{:?}", err) 
        }
    }
    
}