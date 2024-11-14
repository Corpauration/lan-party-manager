import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";

export class LpmngCheckbox extends Component {
  static observedAttributes = ["checked", "disabled"];
  /** @type {(() => void)} */
  onClick;
  constructor() {
    super();

    const register = this.getAttribute("register");
    if (register != null) {
      componentManager.register(register, this);
    }

    this.shadowRoot.getElementById("input").checked =
      this.getAttribute("checked") === "true";
    if (this.hasAttribute("disabled")) {
      this.shadowRoot.getElementById("input").setAttribute("disabled", "");
    }
    this.shadowRoot.getElementById("input").onclick = () => {
      this.setError(false);
      if (this.onClick) {
        this.onClick();
      }
    };
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "checked") {
      this.shadowRoot.getElementById("input").checked = newValue === "true";
    } else if (name === "disabled") {
      if (this.hasAttribute(name)) {
        this.shadowRoot.getElementById("input").setAttribute("disabled", "");
      } else {
        this.shadowRoot.getElementById("input").removeAttribute("disabled");
      }
    }
  }

  /** @returns {boolean} */
  get value() {
    return this.shadowRoot.getElementById("input").checked;
  }

  setError(bool) {
    if (bool) {
      this.shadowRoot.getElementById("label").classList.add("error");
    } else {
      this.shadowRoot.getElementById("label").classList.remove("error");
    }
  }

  setOnClick(callback) {
    this.onClick = callback;
  }
}
