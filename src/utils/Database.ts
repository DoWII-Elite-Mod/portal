import { MongoClient } from 'mongodb';

const client = new MongoClient('mongodb://root:root@localhost:27017');
await client.connect();

async function Matches() {
  return client.db('games').collection('matches');
}

export { Matches };
