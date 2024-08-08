use crate::common::*;

use crate::service::es_service::*;

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
    
    let from_index_name = "consuming_index_prod_new_copy";
    let to_index_name = "consuming_index_prod_new_copy_2";
    
    let mapper_info = match from_es_client.get_cluster_mapping_query(from_index_name).await {
        Ok(res) => res,
        Err(err) => {
            error!("{:?}", err);
            panic!("{:?}", err);
        }
    };

    //println!("{:?}", mapper_info);

    let es_query = json!({
        "query" : {
            "match_all": {}
            },
        "size" : 10000
        }
    );
    
    let search_res = match from_es_client.get_cluster_search_query(es_query, from_index_name).await {
        Ok(search_res) => search_res,
        Err(err) => {
            error!("{:?}", err);
            panic!("{:?}", err);
        }
    };
    
    //info!("{:?}", search_res);
    match to_es_client.set_cluster_mapping_query(to_index_name, mapper_info).await {
        Ok(_) => (),
        Err(err) => {
            error!("{:?}", err);
            panic!("{:?}", err);
        } 
    }
    
    let documents = match search_res["hits"]["hits"].as_array() {
        Some(documents) => documents,
        None => {
            error!("documents is empty");
            panic!("documents is empty");
        }
    };
    
    println!("{:?}", documents);
    
    let hits = &search_res["hits"]["hits"];

    //let mut bulk_body = Vec::new();
    
    // for hit in hits.as_array().unwrap() {
    //     let action = json!({ "index": { "_index": "new_index", "_id": hit["_id"] } });
    //     bulk_body.extend(to_vec(&action)?);
    //     bulk_body.extend(b"\n");
    //     bulk_body.extend(to_vec(&hit["_source"])?);
    //     bulk_body.extend(b"\n");
    // }
    
    // let mut bulk_body = Vec::new();
    
    // for doc in documents {
    //     let index_action = json!({ "index": { "_index": to_index_name, "_id": doc["_id"] } });
    //     bulk_body.push(json!(index_action));
    //     bulk_body.push(doc["_source"].clone());
    // }
    
    
}