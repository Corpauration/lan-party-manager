import { Component } from "../../component.js";
import { Username, ValidString } from "../../../services/validation.js";
import { componentManager } from "../../../services/component-manager.js";
import { Login } from "../../../models/login.js";
import { lpmng } from "../../../services/lpmng.js";
import { router } from "../../../services/router.js";

export class LoginPage extends Component {
  /** @type LpmngTextInput */
  username;
  /** @type LpmngTextInput */
  password;
  /** @type LpmngButton */
  button;

  constructor() {
    super();

    componentManager.listenTo(
      "login-username",
      /** @param {LpmngTextInput} el */ (el) => {
        this.username = el;
        el.setValidator(new Username());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "login-password",
      /** @param {LpmngTextInput} el */ (el) => {
        this.password = el;
        el.setValidator(new ValidString());
        el.setOnEnter(() => this.#submit());
      },
    );
    componentManager.listenTo(
      "login-button",
      /** @param {LpmngButton} el */ (el) => {
        this.button = el;
        el.setOnClick(() => this.#submit());
      },
    );

    if (lpmng.creds != null) {
      router.navigateTo("/");
    }
  }

  connectedCallback() {}

  async #submit() {
    this.username.validate();
    this.password.validate();
    const login = new Login(this.username.value, this.password.value);
    try {
      await lpmng.login(login);
      if (login.username !== "admin") {
        await lpmng.addDevice();
      }
      router.navigateTo("/");
    } catch (e) {
      alert(e);
    }
  }
}
