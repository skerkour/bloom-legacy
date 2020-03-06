/* eslint-disable */
function deepClone(obj: any): any {
  let copy: any = undefined;

  // Handle the 3 simple types, and null or undefined
  if (null == obj || "object" != typeof obj) return obj;

  // Handle Date
  if (obj instanceof Date) {
      copy = new Date();
      copy.setTime(obj.getTime());
      return copy;
  }

  // Handle Array
  if (obj instanceof Array) {
      copy = [];
      for (var i = 0, len = obj.length; i < len; i++) {
          copy[i] = deepClone(obj[i]);
      }
      return copy;
  }

  // Handle Object
  if (obj instanceof Object) {
      copy = {};
      for (var attr in obj) {
          if (obj.hasOwnProperty(attr)) {
            copy[attr] = deepClone(obj[attr]);
          }
      }
      return copy;
  }

  throw new Error("Unable to copy obj! Its type isn't supported.");
}
/* eslint-enable */


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
  level?: Level,
  fields?: Object,
  timestampFieldName?: string,
  messageFieldName?: string,
  levelFieldName?: string,
  hooks?: { (event: Object): void; }[],
};

export interface LoggerInterface {
  config(options: Options): void;
  with(fields: Object): LoggerInterface;
  debug(message: string): void;
  info(message: string): void;
  warn(message: string): void;
  error(message: string): void;
  fatal(message: string): void; // log with the "fatal" level then exit(1)
  track(fields: Object): void;
}

export class Logger implements LoggerInterface {
  private level: Level = Level.DEBUG;
  private fields: any = {};
  private writer = console;
  private insertTimestamp = true;
  private timestampFieldName = 'timestamp';
  private messageFieldName = 'message';
  private levelFieldName = 'level';
  private hooks: { (event: Object): void; }[] = [];


  constructor(options?: Options) {
    if (options) {
      this.config(options);
    }
  }

  /**
   * configure the logger with the given options
   */
  config(options: Options): void {
    if (options.fields) {
      this.fields = options.fields;
    }
    if (options.level) {
      this.level = options.level;
    }
    if (options.timestampFieldName) {
      this.timestampFieldName = options.timestampFieldName;
    }
    if (options.messageFieldName) {
      this.messageFieldName = options.messageFieldName;
    }
    if (options.levelFieldName) {
      this.levelFieldName = options.levelFieldName;
    }
    if (options.hooks) {
      this.hooks = options.hooks;
    }
  }

  /**
   * create a copy of the logger, add the given fields to it and return it
   */
  with(fields: Object): LoggerInterface {
    const newLogger = Object.create(this);
    newLogger.fields = { ...this.fields, ...fields };
    return newLogger;
  }

  /**
   * log an event with the DEBUG level
   */
  debug(message: string): void {
    this.log(Level.DEBUG, message);
  }

  /**
   * log an event with the INFO level
   */
  info(message: string): void {
    this.log(Level.INFO, message);
  }

  /**
   * log an event with the WARN level
   */
  warn(message: string): void {
    this.log(Level.WARN, message);
  }

  /**
   * log an event with the ERROR level
   */
  error(message: string): void {
    this.log(Level.ERROR, message);
  }

  /**
   * log an event with the FATAL level then exit(1)
   */
  fatal(message: string): void {
    this.log(Level.FATAL, message);
    process.exit(1);
  }

  /**
   * log an event without level nor message
   * @param {Object} [fields] - additional fields to add to the event (optional)
   */
  track(fields: Object): void {
    const newLogger = <Logger> this.with(fields);
    newLogger.log(Level.NONE, null);
  }

  private log(level: Level, message: string | Error | null) {
    if (level < this.level) {
      return;
    }

    const event: any = deepClone(this.fields);

    // handle message
    if (message === undefined || message === null) {
      // do nothing
    } else if (typeof message === 'string' && message.length > 0) {
      event[this.messageFieldName] = message;
    } else if (message instanceof Error) {
      event.error_name = message.name;
      event[this.messageFieldName] = message.message;
    } else {
      event[this.messageFieldName] = JSON.stringify(message);
    }

    // handle timestamp
    if (this.insertTimestamp === true) {
      event[this.timestampFieldName] = new Date().toISOString();
    }

    // default case: do not insert level field
    switch (level) {
      case Level.DEBUG:
        event[this.levelFieldName] = 'debug';
        break;
      case Level.INFO:
        event[this.levelFieldName] = 'info';
        break;
      case Level.WARN:
        event[this.levelFieldName] = 'warning';
        break;
      case Level.ERROR:
        event[this.levelFieldName] = 'error';
        break;
      case Level.FATAL:
        event[this.levelFieldName] = 'fatal';
        break;
      default:
        break;
    }

    this.hooks.forEach((hook) => hook(event));

    this.writer.log(event);
  }
}

export const log = new Logger();
