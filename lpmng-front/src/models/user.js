import { Email, Phone, Username, ValidString } from "../services/validation.js";

const usernameValidator = new Username();
const stringValidator = new ValidString();
const emailValidator = new Email();
const phoneValidator = new Phone();

class UserInput {
  /** @type string */
  username;
  /** @type string */
  firstname;
  /** @type string */
  lastname;
  /** @type string */
  email;
  /** @type string */
  password;
  /** @type string */
  phone;

  /**
   * @param {LpmngTextInput|string} username
   * @param {LpmngTextInput|string} firstname
   * @param {LpmngTextInput|string} lastname
   * @param {LpmngTextInput|string} email
   * @param {LpmngTextInput|string} password
   * @param {LpmngTextInput|string} phone
   */
  constructor(username, firstname, lastname, email, password, phone) {
    const u = usernameValidator.validate(username);
    if (u.isErr()) {
      throw u.err();
    }
    this.username = username;

    const f = stringValidator.validate(firstname);
    if (f.isErr()) {
      throw f.err();
    }
    this.firstname = firstname;

    const l = stringValidator.validate(lastname);
    if (l.isErr()) {
      throw l.err();
    }
    this.lastname = lastname;

    const e = emailValidator.validate(email);
    if (e.isErr()) {
      throw e.err();
    }
    this.email = email;

    const p = stringValidator.validate(password);
    if (p.isErr()) {
      throw p.err();
    }
    this.password = password;

    const ph = phoneValidator.validate(phone);
    if (ph.isErr()) {
      throw ph.err();
    }
    this.phone = phone;
  }
}

class UserView {
  /** @type string */
  id;
  /** @type string */
  username;
  /** @type string */
  firstname;
  /** @type string */
  lastname;
  /** @type string */
  email;
  /** @type string */
  phone;
  /** @type string */
  role;
  /** @type boolean */
  isAllowed;

  /**
   * @param {string} id
   * @param {string} username
   * @param {string} firstname
   * @param {string} lastname
   * @param {string} email
   * @param {string} phone
   * @param {string} role
   * @param {boolean} isAllowed
   */
  constructor(
    id,
    username,
    firstname,
    lastname,
    email,
    phone,
    role,
    isAllowed,
  ) {
    this.id = id;
    this.username = username;
    this.firstname = firstname;
    this.lastname = lastname;
    this.email = email;
    this.phone = phone;
    this.role = role;
    this.isAllowed = isAllowed;
  }

  /**
   * @param json
   * @returns {UserView}
   */
  static fromJson(json) {
    return new UserView(
      json.id,
      json.username,
      json.firstname,
      json.lastname,
      json.email,
      json.phone,
      json.role,
      json.is_allowed,
    );
  }
}

export { UserInput, UserView };
