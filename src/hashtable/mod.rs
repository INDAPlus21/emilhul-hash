pub mod hash;
pub mod data;

use anyhow::{Context, Result, bail};
use std::fmt::Debug;
use data::Data;

use self::hash::Hashable;

#[derive(Debug)]
pub struct HashTable<K, V> {
    capacity: u32,
    load_factor: f32,
    size: u32,
    pub table: Vec<Option<Vec<Data<K, V>>>>,
}

impl <K, V> HashTable<K, V> 
where
    K: Hashable + Clone + Debug + Eq,
    V: Clone + Debug
{
    pub fn new(capacity: u32) -> Self {
        let mut table = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            table.push(None);
        }
        
        HashTable {
            capacity: capacity,
            load_factor: 0.75,
            size: 0,
            table: table,
        }
    }

    pub fn insert(&mut self, data: Data<K, V>) -> Result<()> {
        let index = self.compress(data.key.hash());

        match &mut self.table[index] {
            Some(_vec) => {
                _vec.push(data);
                self.size += 1;
            },
            None => {
                self.table[index] = Some(vec![data]);
                self.size += 1;
            }
        }

        if (self.size as f32 / self.capacity as f32 ) >= self.load_factor {
            println!("Resizing table...\r");
            self.capacity *= 2;
            let mut table = HashTable::<K, V>::new(self.capacity);
            println!("Resizing done");
            println!("Rehashing keys...");
            for _bucket in &self.table {
                match _bucket { 
                    Some(_vec) => {
                        for _data in _vec {
                            let data = _data.clone();
                            table.insert(data)
                                .with_context(|| format!("Failed to insert data into new table"))?;
                        }
                    },
                    None => continue
                }
            }
            println!("Rehashing done");
            *self = table;
        }

        Ok(())
    }

    pub fn delete(&mut self, key: K) -> Result<()> {
        let index = self.compress(key.hash());

        match &mut self.table[index] {
            Some(_vec) => {
                if _vec.len() == 1 {
                    self.table[index] = None;
                }
                else {
                    for i in 0.._vec.len() {
                        if key == _vec[i].key {
                            _vec.swap_remove(i);
                        }
                    }
                }
            },
            None => bail!("No such key in table")
        }
        Ok(())
    }

    pub fn print(&self) -> Result<()> {
        let mut msg = format!("\n========== Table ==========\n");
        for _bucket in &self.table {
            match _bucket { 
                Some(_vec) => {
                    let key = &_vec[0].key;
                    msg = format!("{}Key: {:?}, ", msg, key);
                    for _data in _vec {
                        let value = &_data.value;
                        msg = format!("{}Value: {:#?}, ", msg, value);
                    } 
                    msg = format!("{}\n", msg);
                },
                None =>  msg = format!("{}-----\n", msg),
            }
        }
        println!("{}", msg);
        Ok(())
    }

    pub fn compress(&self, hash: u32) -> usize {
        (hash % self.capacity) as usize
    }
}