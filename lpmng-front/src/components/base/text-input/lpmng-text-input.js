import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";

export class LpmngTextInput extends Component {
  static observedAttributes = ["type", "placeholder"];

  /** @type ?Validator */
  validator;
  /** @param {() => void} */
  onEnter;
  /** @param {(string) => void} */
  onInput;

  constructor() {
    super();

    const register = this.getAttribute("register");
    if (register != null) {
      componentManager.register(register, this);
    }

    this.shadowRoot
      .getElementById("in")
      .setAttribute("type", this.getAttribute("type"));
    this.shadowRoot
      .getElementById("in")
      .setAttribute("placeholder", this.getAttribute("placeholder") || "");
    this.attachInternals();
    this.shadowRoot.getElementById("in").addEventListener("input", () => {
      if (this.onInput) {
        this.onInput(this.value);
      }
      this.validate();
    });
    this.shadowRoot.getElementById("input").onclick = () =>
      this.shadowRoot.getElementById("in").focus();
    this.shadowRoot.getElementById("in").onkeydown = (ev) => {
      if (ev.key === "Enter" && this.onEnter) this.onEnter();
    };
  }

  /**
   * @param {?Validator} validator
   */
  setValidator(validator) {
    this.validator = validator;
  }

  validate() {
    if (this.validator) {
      const res = this.validator.validate(
        this.shadowRoot.getElementById("in").value,
      );
      this.shadowRoot.getElementById("main").className = res.isErr()
        ? "error"
        : "";
      if (res.isErr()) {
        this.shadowRoot.getElementById("details").innerText = res.err();
      }
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "type") {
      this.shadowRoot.getElementById("in").setAttribute("type", newValue);
    } else if (name === "placeholder") {
      this.shadowRoot
        .getElementById("in")
        .setAttribute("placeholder", newValue);
    }
  }

  /**
   * @returns {string}
   */
  get value() {
    return this.shadowRoot.getElementById("in").value;
  }

  setOnEnter(callback) {
    this.onEnter = callback;
  }

  setOnInput(callback) {
    this.onInput = callback;
  }
}
