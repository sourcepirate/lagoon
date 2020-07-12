use rusoto_core::credential::ChainProvider;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_core::ByteStream;
use std::io::Read;
use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest};
use md5::compute;
use std::io::{Error, ErrorKind};
use sonic_client::ingest::IngestChan;
use sonic_client::search::SearchChan;

pub struct Store {
    storage_client: StorageClient,
    ingestor: IngestChan,
    searcher: SearchChan,
}

pub struct SonicConfig {
    host: String,
    port: usize,
    password: String,
}

pub struct SearchResult {
    pub doc: String,
    pub hash: String
}

pub struct StorageClient {
    s3_client: S3Client,
}

pub struct QueryConf<'a> {
    pub limit: Option<i32>,
    pub offset: Option<&'a str>
}

impl<'a> Default for QueryConf<'a> {
    fn default() -> QueryConf<'a> {
        QueryConf {
            limit:Some(0),
            offset:None
        }
    }
}

impl SonicConfig {
    pub fn new(host: String, port: usize, password: String) -> Self {
        SonicConfig {
            host,
            port,
            password,
        }
    }

    pub fn ingestor(&self) -> Result<IngestChan, Error> {
        IngestChan::new(&self.host, self.port, &self.password)
    }

    pub fn searcher(&self) -> Result<SearchChan, Error> {
        SearchChan::new(&self.host, self.port, &self.password)
    }
}

impl StorageClient {
    pub fn new(endpoint: String) -> Self {
        let chain = ChainProvider::new();
        let http_client = HttpClient::new().unwrap();
        let region = Region::Custom {
            name: "digitalocean".to_owned(),
            endpoint: endpoint,
        };
        let client = S3Client::new_with(http_client, chain, region);
        StorageClient { s3_client: client }
    }

    pub fn client(&self) -> &S3Client {
        &self.s3_client
    }
}

impl Store {
    pub fn from(client: StorageClient, config: SonicConfig) -> Result<Self, Error> {
        let mut searcher: SearchChan = config.searcher()?;
        let mut ingestor: IngestChan = config.ingestor()?;
        searcher.connect()?;
        ingestor.connect()?;

        Ok(Store {
            ingestor,
            searcher,
            storage_client: client,
        })
    }

    pub async fn put(&mut self, collection: String, text: String) -> Result<String, Error> {
        let hash : String = format!("{:?}", compute(text.clone()));
        let putrequest: PutObjectRequest = PutObjectRequest {
            bucket: collection.clone(),
            key: hash.clone(),
            body: Some(ByteStream::from(text.as_bytes().to_owned())),
            acl: None,
            ..Default::default()
        };
        match self.storage_client.client().put_object(putrequest).await {
            Ok(_) => {
                    self.ingestor.push(
                        &collection,
                        &collection,
                        &hash,
                        &text,
                        None
                    )?;
                return Ok(hash);
            },
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        }
    }

    pub async fn get(&mut self, bucket: String, hash:String) -> Result<String, Error> {
        match self.storage_client.client().get_object(
            GetObjectRequest{
                bucket: bucket,
                key: hash,
                ..Default::default()
            }
        ).await {
            Ok(stream) => {
                if !stream.body.is_some(){
                    return Ok(String::from(""));
                } else {
                    let bytes = stream.body.unwrap();
                    let mut data = bytes.into_blocking_read();
                    let mut content = String::new();
                    data.read_to_string(&mut content).map(|_|content)
                }
            },
            Err(e) => Err(Error::new(ErrorKind::Other, e))
        } 
    }

    pub async fn search<'a>(&mut self, bucket: String, term: String, queryconf: QueryConf<'a>) -> Result<Vec<SearchResult>, Error> {
        let hashes : Vec<String> = self.searcher.query(
            &bucket,
            &bucket,
            &term,
            queryconf.limit,
            queryconf.offset
        )?;
        let mut documents : Vec<SearchResult>= Vec::new();
        for hash in hashes.into_iter() {
            let val = hash.clone();
            let result = self.get(bucket.clone(), val).await;
            if !result.is_err() {
                documents.push(SearchResult{doc:result.unwrap(), hash: hash.clone()});
            }
        }
        Ok(documents)
    }
}
