use crate::common::*;

use crate::service::es_service::*;

/*
    
*/
pub async fn re_index_start(from_es: &EsHelper, to_es: &EsHelper, old_index_name: &str, new_index_name: &str) -> Result<(), anyhow::Error> {
    
    let mapper_info = from_es.get_cluster_mapping_query(old_index_name).await?;
    
    println!("{:?}", mapper_info);

    let es_query = json!({
        "query" : {
            "match_all": {}
            },
        "size" : 10000
        }
    );
    
    let search_res = from_es.get_cluster_search_query(es_query, old_index_name).await?;

    to_es.set_cluster_mapping_query(old_index_name, new_index_name, mapper_info).await?;
    
    let documents = search_res["hits"]["hits"].as_array().ok_or_else(|| anyhow!("There is no query result for the 'documents' variable. - re_index_start()"))?;
    
    let mut ops: Vec<BulkOperation<Value>> = Vec::new();
    
    for doc in documents {

        let doc_id = doc["_id"].as_str().ok_or_else(|| anyhow!("There is no query result for the 'doc_id' variable. - re_index_start()"))?;
        let source = doc["_source"].clone();
         
        let operation = BulkOperation::index(source)
            .id(doc_id)
            .index(new_index_name);
        
        ops.push(operation.into());
    }
    
    println!("{:?}", ops.len());

    to_es.set_cluster_bulk_query(new_index_name, &ops).await?;
    
    Ok(())
}