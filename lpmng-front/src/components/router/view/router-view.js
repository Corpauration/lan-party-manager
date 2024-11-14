import { Component } from "../../component.js";
import { router } from "../../../services/router.js";

export class RouterView extends Component {
  constructor() {
    super();

    router.addListener((router) => this.changeView(router));
    this.changeView(router);
  }

  /**
   * @param {Router} router
   */
  changeView(router) {
    for (let child of this.shadowRoot.children) {
      this.shadowRoot.removeChild(child);
    }

    this.shadowRoot.appendChild(router.element);
  }
}
