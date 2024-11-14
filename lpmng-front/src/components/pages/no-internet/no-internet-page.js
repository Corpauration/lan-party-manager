import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";
import { lpmng } from "../../../services/lpmng.js";
import { router } from "../../../services/router.js";

export class NoInternetPage extends Component {
  /** @type LpmngButton */
  logout;
  /** @type LpmngButton */
  refresh;

  constructor() {
    super();

    componentManager.listenTo(
      "ni-logout",
      /** @param {LpmngButton} el */ (el) => {
        this.logout = el;
        el.setOnClick(async () => {
          await lpmng.logout();
        });
      },
    );

    componentManager.listenTo(
      "ni-refresh",
      /** @param {LpmngButton} el */ (el) => {
        this.refresh = el;
        el.setOnClick(() => this.getInternetIfPossible());
      },
    );
  }

  async getInternetIfPossible() {
    if (lpmng.creds == null) {
      return router.navigateTo("/login");
    }
    const user = await lpmng.getUser(lpmng.creds.userId);
    if (user.isAllowed) {
      const devices = await lpmng.getUserDevices(lpmng.creds.userId);
      if (
        devices.length === 0 ||
        devices.find((device) => !device.internet) !== undefined
      ) {
        await lpmng.addDevice();
      } else {
        router.navigateTo("/");
      }
    }
  }
}
