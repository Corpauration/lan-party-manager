import { Username, ValidString } from "../services/validation.js";

const usernameValidator = new Username();
const stringValidator = new ValidString();

class Login {
  /** @type string */
  username;
  /** @type string */
  password;

  /**
   * @param {string} username
   * @param {string} password
   */
  constructor(username, password) {
    const u = usernameValidator.validate(username);
    if (u.isErr()) {
      throw u.err();
    }
    this.username = username;
    const p = stringValidator.validate(password);
    if (p.isErr()) {
      throw p.err();
    }
    this.password = password;
  }

  /** @returns {{password: string, username: string}} */
  toJson() {
    return { username: this.username, password: this.password };
  }
}

class Credentials {
  /** @type string */
  biscuit;
  /** @type string */
  role;
  /** @type string */
  userId;

  constructor(biscuit, role, userId) {
    this.biscuit = biscuit;
    this.role = role;
    this.userId = userId;
  }

  /**
   * @param {{"biscuit": string, "role": string, "user_id": string}} json
   * @returns {Credentials}
   */
  static fromJson(json) {
    return new Credentials(json.biscuit, json.role, json.user_id);
  }
}

export { Login, Credentials };
