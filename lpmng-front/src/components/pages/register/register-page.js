import { Component } from "../../component.js";
import {
  Email,
  Phone,
  Username,
  ValidString,
  Validator,
  notEmpty,
} from "../../../services/validation.js";
import { componentManager } from "../../../services/component-manager.js";
import { lpmng } from "../../../services/lpmng.js";
import { UserInput } from "../../../models/user.js";
import { Err, Ok } from "../../../models/result.js";
import { router } from "../../../services/router.js";

export class RegisterPage extends Component {
  /** @type LpmngTextInput */
  username;
  /** @type LpmngTextInput */
  firstname;
  /** @type LpmngTextInput */
  lastname;
  /** @type LpmngTextInput */
  email;
  /** @type LpmngTextInput */
  phone;
  /** @type LpmngTextInput */
  password;
  /** @type LpmngTextInput */
  confirmPassword;
  /** @type LpmngCheckbox */
  agree;
  /** @type LpmngButton */
  button;

  constructor() {
    super();

    componentManager.listenTo(
      "register-username",
      /** @param {LpmngTextInput} el */ (el) => {
        this.username = el;
        el.setValidator(new Username());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-firstname",
      /** @param {LpmngTextInput} el */ (el) => {
        this.firstname = el;
        el.setValidator(new ValidString());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-lastname",
      /** @param {LpmngTextInput} el */ (el) => {
        this.lastname = el;
        el.setValidator(new ValidString());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-email",
      /** @param {LpmngTextInput} el */ (el) => {
        this.email = el;
        el.setValidator(new Email());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-phone",
      /** @param {LpmngTextInput} el */ (el) => {
        this.phone = el;
        el.setValidator(new Phone());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-password",
      /** @param {LpmngTextInput} el */ (el) => {
        this.password = el;
        el.setValidator(new ValidString());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "register-confirm-password",
      /** @param {LpmngTextInput} el */ (el) => {
        this.confirmPassword = el;
        el.setValidator(
          new Validator([
            notEmpty,
            (str) => {
              if (str !== this.password.value) {
                return new Err("mot de passe différent");
              } else {
                return new Ok(null);
              }
            },
          ]),
        );
        el.setOnEnter(() => this.#submit());
      },
    );

    componentManager.listenTo(
      "register-agree",
      /** @param {LpmngButton} el */ (el) => {
        this.agree = el;
      },
    );

    componentManager.listenTo(
      "register-button",
      /** @param {LpmngButton} el */ (el) => {
        this.button = el;
        el.setOnClick(() => this.#submit());
      },
    );

    if (lpmng.creds != null) {
      router.navigateTo("/");
    }

    componentManager.listenTo(
      "table",
      /** @param {LpmngTable} el */ (el) => {
        console.log(el);
      },
    );
  }

  async #submit() {
    this.username.validate();
    this.firstname.validate();
    this.lastname.validate();
    this.email.validate();
    this.phone.validate();
    this.password.validate();
    this.confirmPassword.validate();
    this.agree.setError(!this.agree.value);
    if (!this.agree.value) {
      throw "règlement non accepté";
    }
    if (this.password.value !== this.confirmPassword.value) {
      throw "mots de passe différents";
    }

    const user = new UserInput(
      this.username.value,
      this.firstname.value,
      this.lastname.value,
      this.email.value,
      this.password.value,
      this.phone.value,
    );
    try {
      await lpmng.register(user);
      await lpmng.addDevice();
      router.navigateTo("/");
    } catch (e) {
      alert(e);
    }
  }
}
