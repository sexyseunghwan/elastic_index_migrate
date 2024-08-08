use crate::common::*;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct EsHelper {
    mon_es_pool: Vec<EsObj>
}

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct EsObj {
    es_host: String,
    es_pool: Elasticsearch
}


impl EsHelper {

    /* 
        Constructor
    */
    pub fn new(es_url_vec: Vec<String>, es_id: &str, es_pw: &str) -> Result<Self, anyhow::Error> {
        
        let mut mon_es_clients: Vec<EsObj> = Vec::new();
    
        for url in es_url_vec {
    
            let parse_url = format!("http://{}:{}@{}", es_id, es_pw, url);
    
            let es_url = Url::parse(&parse_url)?;
            let conn_pool = SingleNodeConnectionPool::new(es_url);
            let transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(5,0))
                .build()?;
            
            mon_es_clients.push(EsObj::new(url, Elasticsearch::new(transport)));
        }
        
        Ok(EsHelper{mon_es_pool: mon_es_clients})
    }
    

    /*
        Functions that handle queries at the Elasticsearch Cluster LEVEL - mapping
    */
    pub async fn get_cluster_mapping_query(&self, index_name: &str) -> Result<Value, anyhow::Error> {
        
        for es_obj in self.mon_es_pool.iter() {

            match es_obj.get_node_mapping_query(index_name).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    error!("{:?}", err);      
                    continue;
                }
            }   
        }
        
        Err(anyhow!("All Elasticsearch connections failed"))
    }
    
    /*
    
    */
    pub async fn set_cluster_mapping_query(&self, index_name: &str, mapper_info: Value) -> Result<(), anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.set_node_mapping_query(index_name, &mapper_info).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    error!("{:?}", err);      
                    continue;
                }
            }   
        }
        
        Err(anyhow!("All Elasticsearch connections failed"))
    }
    
    /*
    
    */
    // pub async fn set_cluster_bulk_query(&self, ) -> Result<(), anyhow::Error> {

    //     for es_obj in self.mon_es_pool.iter() {

    //         match es_obj.set_node_mapping_query(index_name, &mapper_info).await {
    //             Ok(resp) => return Ok(resp),
    //             Err(err) => {
    //                 error!("{:?}", err);      
    //                 continue;
    //             }
    //         }   
    //     }
        
    //     Err(anyhow!("All Elasticsearch connections failed"))
    // }

    
    /*
        Functions that handle queries at the Elasticsearch Cluster LEVEL - search
    */
    pub async fn get_cluster_search_query(&self, es_query: Value, index_name: &str) -> Result<Value, anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.get_node_search_query(&es_query, index_name).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    error!("{:?}", err);      
                    continue;
                }
            }   
        }
        
        Err(anyhow!("All Elasticsearch connections failed"))
    }
    
}


impl EsObj {


    /*
        Function that EXECUTES elasticsearch queries - mapping
    */
    async fn get_node_mapping_query(&self, index_name: &str) -> Result<Value, anyhow::Error> {

        let mapping_response = self.es_pool.indices().get_mapping(IndicesGetMappingParts::Index(&[index_name])).send().await?;
        let mapping_body = mapping_response.json::<serde_json::Value>().await?;
                  
        Ok(mapping_body)
    }

    
    /*

    */
    async fn set_node_mapping_query(&self, index_name: &str, mapper_info: &Value) -> Result<(), anyhow::Error> {

        self.es_pool.indices().create(IndicesCreateParts::Index(index_name))
            .body(mapper_info).send().await?;
    
        Ok(())
    }

    /*
    
    */
    // async fn set_node_bulk_query(&self, index_name: &str, bulk_body: Vec<Value>) -> Result<(), anyhow::Error> {

    //     self.es_pool.bulk(BulkParts::Index(index_name))
    //         .body(bulk_body)
    //         .send()
    //         .await?;

    //     Ok(())
    // }

    /*
        Function that EXECUTES elasticsearch queries - search
    */
    async fn get_node_search_query(&self, es_query: &Value, index_name: &str) -> Result<Value, anyhow::Error> {

        // Response Of ES-Query
        let response = self.es_pool
            .search(SearchParts::Index(&[index_name]))
            .body(es_query)
            .send()
            .await?;
    
        if response.status_code().is_success() { 
            let response_body = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("response status is failed"))
        }
    }

}
