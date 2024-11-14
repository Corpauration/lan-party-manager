import { Component } from "../../component.js";
import { router } from "../../../services/router.js";

export class RouterLink extends Component {
  static observedAttributes = ["href", "invisible"];
  constructor() {
    super();

    this.shadowRoot.getElementById("main").href = this.getAttribute("href");
    this.shadowRoot.getElementById("main").className = this.hasAttribute(
      "invisible",
    )
      ? "invisible"
      : "";
    this.shadowRoot.getElementById("main").onclick = (e) => {
      e.preventDefault();
      router.navigateTo(this.shadowRoot.getElementById("main").href);
    };
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "href") {
      this.shadowRoot.getElementById("main").href = newValue;
    } else if (name === "invisible") {
      this.shadowRoot.getElementById("main").className = this.hasAttribute(
        "invisible",
      )
        ? "invisible"
        : "";
    }
  }
}
