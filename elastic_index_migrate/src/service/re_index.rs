use crate::common::*;

use crate::service::es_service::*;

/*
    
*/
pub async fn re_index_start(from_es: &EsHelper, to_es: &EsHelper, index_name: &str, new_index_name: &str) -> Result<(), anyhow::Error> {
    
    let mapper_info = from_es.get_cluster_mapping_query(index_name).await?;

    let es_query = json!({
        "query" : {
            "match_all": {}
            },
        "size" : 10000
        }
    );
    
    let search_res = from_es.get_cluster_search_query(es_query, index_name).await?;

    to_es.set_cluster_mapping_query(new_index_name, mapper_info).await?;

    let documents = search_res["hits"]["hits"].as_array().ok_or_else(|| anyhow!("There is no query result for the 'documents' variable. - re_index_start()"))?;
    
    let mut ops: Vec<BulkOperation<Value>> = Vec::new();

    for doc in documents {
        
        let operation = BulkOperation::index(doc["_source"].clone())
            .id(doc["_id"].as_str().unwrap())
            .index(doc["_index"].as_str().unwrap());
        
        ops.push(operation.into());
    }
    
    
    
    Ok(())
}