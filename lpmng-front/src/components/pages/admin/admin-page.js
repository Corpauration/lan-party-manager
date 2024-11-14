import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";
import { Header } from "../../base/table/lpmng-table.js";
import { lpmng } from "../../../services/lpmng.js";

export class AdminPage extends Component {
  /** @type LpmngTable */
  table;

  constructor() {
    super();

    componentManager.listenTo(
      "users-table",
      /** @param {LpmngTable} el */ (el) => {
        this.table = el;
        el.setHeaders(
          new Map([
            ["id", new Header("Id")],
            ["username", new Header("Nom d'utilisateur")],
            ["firstname", new Header("Prénom")],
            ["lastname", new Header("Nom")],
            ["email", new Header("Email")],
            ["phone", new Header("Téléphone")],
            ["role", new Header("Role")],
            [
              "isAllowed",
              new Header("Internet ?", true, false, "boolean", (user) =>
                this.#switchInternet(user),
              ),
            ],
          ]),
        );

        lpmng.getUsers().then((users) => el.setData(users));
      },
    );
  }

  /**
   * @param {UserView} user
   * @returns {Promise<void>}
   */
  async #switchInternet(user) {
    console.log(user);
    await lpmng.setInternet(user.id, !user.isAllowed);
    user.isAllowed = !user.isAllowed;
    this.table.updateData("id", user);
  }
}
