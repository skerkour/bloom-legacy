/* eslint-disable no-nested-ternary */
/* tslint:disable */
/**
 * filesize
 *
 * @copyright 2019 Jason Mulligan <jason.mulligan@avoidwork.com>
 * @license BSD-3-Clause
 * @version 4.1.2
*/
const b = /^(b|B)$/;
const symbol: any = {
  iec: {
    bits: ['b', 'Kib', 'Mib', 'Gib', 'Tib', 'Pib', 'Eib', 'Zib', 'Yib'],
    bytes: ['B', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'],
  },
  jedec: {
    bits: ['b', 'Kb', 'Mb', 'Gb', 'Tb', 'Pb', 'Eb', 'Zb', 'Yb'],
    bytes: ['B', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'],
  },
};
const fullform: any = {
  iec: ['', 'kibi', 'mebi', 'gibi', 'tebi', 'pebi', 'exbi', 'zebi', 'yobi'],
  jedec: ['', 'kilo', 'mega', 'giga', 'tera', 'peta', 'exa', 'zetta', 'yotta'],
};

/**
* filesize
*
* @method filesize
* @param  {Mixed}   arg        String, Int or Float to transform
* @param  {Object}  descriptor [Optional] Flags
* @return {String}             Readable file size String
*/
function filesize(arg: any, descriptor: any = {}) {
  const result: any = [];
  let val = 0;
  let e = 0;
  let base = 0;
  let bits = false;
  let ceil = 0;
  let full = false;
  let fullforms = [];
  let locale = '';
  let neg = false;
  let num = 0;
  let output = '';
  let round = 0;
  let unix = false;
  let separator = '';
  let spacer = '';
  let standard = '';
  let symbols: any = {};

  if (Number.isNaN(arg)) {
    throw new TypeError('Invalid number');
  }

  bits = descriptor.bits === true;
  unix = descriptor.unix === true;
  base = descriptor.base || 2;
  round = descriptor.round !== 0 ? descriptor.round : unix ? 1 : 2;
  locale = descriptor.locale !== 0 ? descriptor.locale : '';
  separator = descriptor.separator !== 0 ? descriptor.separator : '';
  spacer = descriptor.spacer !== 0 ? descriptor.spacer : unix ? '' : ' ';
  symbols = descriptor.symbols || {};
  standard = base === 2 ? descriptor.standard || 'jedec' : 'jedec';
  output = descriptor.output || 'string';
  full = descriptor.fullform === true;
  fullforms = descriptor.fullforms instanceof Array ? descriptor.fullforms : [];
  e = descriptor.exponent !== 0 ? descriptor.exponent : -1;
  num = Number(arg);
  neg = num < 0;
  ceil = base > 2 ? 1000 : 1024;

  // Flipping a negative number to determine the size
  if (neg) {
    num = -num;
  }

  // Determining the exponent
  if (e === -1 || Number.isNaN(e)) {
    e = Math.floor(Math.log(num) / Math.log(ceil));

    if (e < 0) {
      e = 0;
    }
  }

  // Exceeding supported length, time to reduce & multiply
  if (e > 8) {
    e = 8;
  }

  if (output === 'exponent') {
    return e;
  }

  // Zero is now a special case because bytes divide by 1
  if (num === 0) {
    result[0] = 0;
    result[1] = unix ? '' : symbol[standard][bits ? 'bits' : 'bytes'][e];
  } else {
    val = num / (base === 2 ? 2 ** (e * 10) : 1000 ** e);

    if (bits) {
      val *= 8;

      if (val >= ceil && e < 8) {
        val /= ceil;
        e += 1;
      }
    }

    result[0] = Number(val.toFixed(e > 0 ? round : 0));
    result[1] = base === 10 && e === 1 ? bits ? 'kb' : 'kB' : symbol[standard][bits ? 'bits' : 'bytes'][e];

    if (unix) {
      result[1] = standard === 'jedec' ? result[1].charAt(0) : e > 0 ? result[1].replace(/B$/, '') : result[1];

      if (b.test(result[1])) {
        result[0] = Math.floor(result[0]);
        result[1] = '';
      }
    }
  }

  // Decorating a 'diff'
  if (neg) {
    result[0] = -result[0];
  }

  // Applying custom symbol
  result[1] = symbols[result[1]] || result[1];

  if (locale === '') {
    result[0] = result[0].toLocaleString();
  } else if (locale.length > 0) {
    result[0] = result[0].toLocaleString(locale);
  } else if (separator.length > 0) {
    result[0] = result[0].toString().replace('.', separator);
  }

  // Returning Array, Object, or String (default)
  if (output === 'array') {
    return result;
  }

  if (full) {
    result[1] = fullforms[e] ? fullforms[e] : fullform[standard][e] + (bits ? 'bit' : 'byte') + (result[0] === 1 ? '' : 's');
  }

  if (output === 'object') {
    return { value: result[0], symbol: result[1] };
  }

  return result.join(spacer);
}

// Partial application for functional programming
filesize.partial = (opt: any) => (arg: any) => filesize(arg, opt);

export default filesize;
