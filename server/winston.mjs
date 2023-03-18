import winston from 'winston';
import morgan from 'morgan';

const levels = {
  error: 0,
  warn: 1,
  info: 2,
  http: 3,
  debug: 4
};

const colors = {
  error: 'red',
  warn: 'yellow',
  info: 'green',
  http: 'magenta',
  debug: 'blue'
};

const format = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
  winston.format.colorize({ all: true }),
  winston.format.errors({ stack: true }),
  winston.format.splat(),
  winston.format.printf(
    (info) =>
      `${info.timestamp} [${info.level}] (${info.service}) ${info.stack || info.message
      }`
  )
);

winston.addColors(colors);

export const logger = winston.createLogger({
  level: 'debug',
  transports: [new winston.transports.Console()],
  defaultMeta: { service: 'root' },
  format,
  levels
});

// Morgan middleware setup
const stream = {
  write: (message) => logger.http(message)
};

export const morganMiddleware = morgan(
  ':method :url :status :res[content-length] - :response-time ms',
  { stream }
);
