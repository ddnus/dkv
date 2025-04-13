use std::path::PathBuf;
use rocksdb::{DB, Options, IteratorMode, Direction};
use crate::error::Error;

pub struct KV {
    db: DB
}

impl KV {
    /// 创建一个新的 KV 数据库
    pub fn new(path: &PathBuf) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(KV { db })
    }

    /// 插入一个键值对
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db.put(key, value)?;
        Ok(())
    }

    /// 获取一个键的值
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<Vec<u8>>, Error> {
        let data = self.db.get(key)?;
        Ok(data)
    }

    /// 删除一个键
    pub fn delete<K: AsRef<[u8]>>(&self, key: K) -> Result<(), Error> {
        self.db.delete(key)?;
        Ok(())
    }

    /// 批量写入
    pub fn batch_write<K, V>(&self, kvs: Vec<(K, V)>) -> Result<(), Error>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        let mut batch = rocksdb::WriteBatch::default();
        for (key, value) in kvs {
            batch.put(key, value);
        }
        self.db.write(batch)?;
        Ok(())
    }

    /// 遍历所有键值对
    pub fn range(&self) -> impl Iterator<Item = (Box<[u8]>, Box<[u8]>)> + '_ {
        self.db
            .iterator(IteratorMode::Start)
            .map(|item| item.unwrap())
    }

    /// 按前缀查找键值对
    pub fn prefix_search(&self, prefix: &[u8]) -> impl Iterator<Item = (Box<[u8]>, Box<[u8]>)> + '_ {
        self.db
            .prefix_iterator(prefix)
            .map(|item| item.unwrap())
    }

    /// 模糊查找键 (包含特定字符串的键)
    pub fn key_like(&self, pattern: &str) -> Vec<(Box<[u8]>, Box<[u8]>)> {
        self.range()
            .filter(|(key, _)| {
                String::from_utf8_lossy(key).contains(pattern)
            })
            .collect()
    }

    /// 范围查询 (start..end)
    pub fn range_query<'a>(&'a self, start: &'a [u8], end: &'a [u8]) -> impl Iterator<Item = (Box<[u8]>, Box<[u8]>)> + 'a {
        self.db
            .iterator(IteratorMode::From(start, Direction::Forward))
            .take_while(move |item| {
                let (key, _) = item.as_ref().unwrap();
                key.as_ref() < end
            })
            .map(|item| item.unwrap())
    }

    // 如果需要 Vec<u8> 版本的方法
    pub fn range_query_vec<'a>(&'a self, start: &'a [u8], end: &'a [u8]) -> impl Iterator<Item = (Vec<u8>, Vec<u8>)> + 'a {
        self.range_query(start, end)
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
    }

    /// 获取所有键
    pub fn keys(&self) -> Vec<Box<[u8]>> {
        self.range()
            .map(|(key, _)| key)
            .collect()
    }

    /// 获取所有值
    pub fn values(&self) -> Vec<Box<[u8]>> {
        self.range()
            .map(|(_, value)| value)
            .collect()
    }
}