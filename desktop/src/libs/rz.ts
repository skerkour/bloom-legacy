export enum Level {
  DEBUG,
  INFO,
  WARN,
  ERROR,
  FATAL,
  NONE,
  NOOP,
}

export type Options = {
  level: Level,
  fields?: Object,
};

export interface LoggerInterface {
  config(options: Options): void;
  with(fields: any): LoggerInterface;
  debug(message: string): void;
  info(message: string): void;
  warn(message: string): void;
  error(message: string): void;
  fatal(message: string): void; // log with the "fatal" level then exit(1)
  track(fields: any): void;
}

export class Logger implements LoggerInterface {
  config(message: Options): void {
    console.log(message);
  }

  with(fields: any): LoggerInterface {
    console.log(fields);
    return this;
  }

  debug(message: string): void {
    console.log(message);
  }

  info(message: string): void {
    console.log(message);
  }

  warn(message: string): void {
    console.log(message);
  }

  error(message: string): void {
    console.log(message);
  }

  fatal(message: string): void {
    console.log(message);
  }

  track(fields: any): void {
    console.log(fields);
  }
}

const log = new Logger();

export { log };
