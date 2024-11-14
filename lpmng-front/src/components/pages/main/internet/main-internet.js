import { Component } from "../../../component.js";
import { lpmng } from "../../../../services/lpmng.js";
import { componentManager } from "../../../../services/component-manager.js";

export class MainInternet extends Component {
  constructor() {
    super();

    componentManager.listenTo(
      "main-logout",
      /** @param {LpmngButton} el */ (el) => {
        el.setOnClick(() => lpmng.logout());
      },
    );

    this.#loadIdentity();
  }

  async #loadIdentity() {
    const user = await lpmng.getUser(lpmng.creds.userId);

    this.shadowRoot.getElementById("firstname").innerText = user.firstname;

    this.shadowRoot.getElementById("video").onclick = () =>
      this.shadowRoot.getElementById("video").play();
  }
}
