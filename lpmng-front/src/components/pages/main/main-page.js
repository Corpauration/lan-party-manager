import { Component } from "../../component.js";
import { lpmng } from "../../../services/lpmng.js";
import { router } from "../../../services/router.js";

export class MainPage extends Component {
  constructor() {
    super();

    this.#checkIfInternet().catch((e) => console.log(e));
  }

  connectedCallback() {}

  async #checkIfInternet() {
    if (lpmng.creds == null) {
      return router.navigateTo("/login");
    }
    const user = await lpmng.getUser(lpmng.creds.userId);
    if (user.isAllowed) {
      const devices = await lpmng.getUserDevices(lpmng.creds.userId);
      if (
        devices.length > 0 &&
        devices.find((device) => device.internet) !== undefined
      ) {
        return this.shadowRoot.appendChild(
          document.createElement("main-internet"),
        );
      }
    }
    router.navigateTo("/no-internet");
  }
}
