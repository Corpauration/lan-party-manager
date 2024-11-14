import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";

export class LpmngButton extends Component {
  static observedAttributes = ["type"];
  /** @param {() => void} */
  onClick;

  constructor() {
    super();

    const register = this.getAttribute("register");
    if (register != null) {
      componentManager.register(register, this);
    }

    this.shadowRoot.getElementById("main").className =
      this.getAttribute("type") || "normal";
    this.shadowRoot.getElementById("main").onclick = () => {
      if (this.onClick) {
        this.onClick();
      }
    };
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "type") {
      this.shadowRoot.getElementById("main").className = newValue || "normal";
    }
  }

  /**
   * @param {() => void} callback
   */
  setOnClick(callback) {
    this.onClick = callback;
  }
}
