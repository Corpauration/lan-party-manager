import { Err, Ok } from "../models/result.js";

const emailRegex =
  /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/;
const phoneRegex =
  /0[67][\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}|\+33[\s.]?[67][\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}[\s.]?\d{2}/;

/**
 * @param {string} str
 * @return {Result.<null, string>}
 */
function notEmpty(str) {
  if (str.length === 0) {
    return new Err("texte vide");
  } else {
    return new Ok(null);
  }
}

/**
 * @param {string} str
 * @return {Result.<null, string>}
 */
function malformed(str) {
  if (str.startsWith(" ")) {
    return new Err("texte malformé");
  } else {
    return new Ok(null);
  }
}

/**
 * @param {string} str
 * @return {Result.<null, string>}
 */
function len(str) {
  if (str.length > 128) {
    return new Err("texte trop long");
  } else {
    return new Ok(null);
  }
}

class Validator {
  /** @type {Array.<(string) => Result.<null, string>>} */
  #rules;

  /**
   * @param {Array.<function(string): Result.<null, string>>} rules
   */
  constructor(rules) {
    this.#rules = rules;
  }

  /**
   * @param {string} str
   * @return {Result.<string, string>}
   */
  validate(str) {
    const res = this.#rules.map((f) => f(str).err()).find((e) => e != null);
    if (res == null) {
      return new Ok(str);
    } else {
      return new Err(res);
    }
  }
}

class ValidString extends Validator {
  constructor() {
    super([notEmpty, malformed, len]);
  }
}

class Username extends Validator {
  constructor() {
    super([
      notEmpty,
      malformed,
      len,
      (str) => {
        if (str.includes(" ")) {
          return new Err("le nom d'utilisateur contient des espaces");
        } else {
          return new Ok(null);
        }
      },
    ]);
  }
}

class Email extends Validator {
  constructor() {
    super([
      (str) => {
        if (!emailRegex.test(str)) {
          return new Err("email invalide");
        } else {
          return new Ok(null);
        }
      },
    ]);
  }
}

class Phone extends Validator {
  constructor() {
    super([
      (str) => {
        if (!phoneRegex.test(str)) {
          return new Err("numéro de téléphone invalide");
        } else {
          return new Ok(null);
        }
      },
    ]);
  }
}

export {
  Validator,
  ValidString,
  Username,
  Email,
  Phone,
  notEmpty,
  malformed,
  len,
};
