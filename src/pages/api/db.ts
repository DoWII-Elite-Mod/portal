import type { QueryResultRow } from 'pg';
import pg from 'pg';

const { Pool } = pg;

const pool = new Pool({
  database: process.env.DB ?? 'elitemod',
  host: process.env.HOST ?? 'localhost',
  user: process.env.DB_USER ?? 'postgres',
  port: (process.env.DB_PORT as number | undefined) ?? 5432,
  password: process.env.DB_PASSWORD ?? 'postgres'
});

pool.on('connect', (client) => {
  client.query('SET search_path TO elitemod,public;');
});

pool
  .query('SELECT 1')
  .then(() => console.log('DB connection open'))
  .catch((error) => console.error(error?.message ?? error));

export function query<T extends QueryResultRow>(sql: string, params?: any[]) {
  console.debug(`SQL Query: ${sql}`);
  if (params) {
    console.debug(`SQL Query Parameters: ${params}`);
  }

  return new Promise((resolve, reject) => {
    if (params) {
      pool.query<T>(sql, params, (error, results) => {
        if (error) {
          console.error(`SQL query error: ${error}`);
          reject(error);
        }

        resolve(results);
      });
    } else {
      pool.query<T>(sql, (error, results) => {
        if (error) {
          console.error(`SQL query error: ${error}`);
          reject(error);
        }

        resolve(results);
      });
    }
  });
}
