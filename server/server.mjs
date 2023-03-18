import express from 'express';
import { handler as ssrHandler } from '../dist/server/entry.mjs';
import helmet from 'helmet';
import { morganMiddleware } from './winston.mjs';

const app = express();

app.use(morganMiddleware);
app.use(helmet());
app.use(express.static('dist/client/'));
app.use(ssrHandler);

const port = process.env.API_PORT ?? 4000;
app.listen(port, () => console.log('Listening...'));
