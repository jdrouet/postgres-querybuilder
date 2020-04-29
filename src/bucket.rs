use postgres_types::ToSql;

type BucketValue = dyn ToSql + Sync;

pub struct Bucket {
  content: Vec<Box<BucketValue>>,
}

impl Bucket {
  pub fn new() -> Bucket {
    Bucket { content: vec![] }
  }

  pub fn get_refs(self) -> Vec<&'static BucketValue> {
    let mut args: Vec<&BucketValue> = vec![];
    for item in self.content {
      args.push(Box::leak(item));
    }
    args
  }

  pub fn push<T: 'static + ToSql + Sync + Clone>(&mut self, value: T) -> usize {
    self.content.push(Box::new(value));
    self.content.len()
  }

  pub fn len(&self) -> usize {
    self.content.len()
  }
}
