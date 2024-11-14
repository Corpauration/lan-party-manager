import { Component } from "../../component.js";
import { componentManager } from "../../../services/component-manager.js";

class Header {
  /** @type string */
  name;
  /** @type boolean */
  editable;
  /** @type boolean */
  searchable;
  /** @type string */
  type;
  /** @type {(any) => void} */
  callback;

  /**
   * @param {string} name
   * @param {boolean} editable
   * @param {boolean} searchable
   * @param {string} type
   * @param {function(*): void} callback
   */
  constructor(
    name,
    editable = false,
    searchable = true,
    type = "string",
    callback = null,
  ) {
    this.name = name;
    this.editable = editable;
    this.searchable = searchable;
    this.type = type;
    this.callback = callback;
  }
}

class LpmngTable extends Component {
  static observedAttributes = ["title"];
  /** @type {Map.<string, Header>} */
  headers;
  data = [];

  constructor() {
    super();

    this.shadowRoot.getElementById("title").innerText =
      this.getAttribute("title");
    const input = document.createElement("lpmng-text-input");
    input.setAttribute("placeholder", "Recherche");
    input.setAttribute("type", "text");
    input.setOnInput((query) => this.search(query));
    this.shadowRoot.getElementById("input").appendChild(input);

    const register = this.getAttribute("register");
    if (register != null) {
      componentManager.register(register, this);
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (name === "title") {
      this.shadowRoot.getElementById("title").innerText = newValue;
    }
  }

  /**
   * @param {Map<string, Header>} headers
   */
  setHeaders(headers) {
    let headerElement = this.shadowRoot.getElementById("headers");
    this.headers = headers;
    for (let header of this.headers.entries()) {
      const cell = document.createElement("th");
      cell.innerText = header[1].name;
      headerElement.appendChild(cell);
    }
  }

  setData(data, update = true) {
    if (update) {
      this.data = data;
    }
    let dataElement = this.shadowRoot.getElementById("data");
    dataElement.innerHTML = "";

    for (let d of data) {
      const line = document.createElement("tr");
      for (let header of this.headers.entries()) {
        const cell = document.createElement("td");
        if (header[1].type === "string") {
          cell.innerText = d[header[0]];
        } else if (header[1].type === "boolean") {
          const checkbox = document.createElement("lpmng-checkbox");
          checkbox.setAttribute("checked", d[header[0]]);
          if (!header[1].editable) {
            checkbox.setAttribute("disabled", "");
          } else {
            checkbox.setOnClick(() => header[1].callback(d));
          }
          cell.appendChild(checkbox);
        }

        line.appendChild(cell);
      }
      dataElement.appendChild(line);
    }
  }

  updateData(key, data) {
    for (let i = 0; i < this.data.length; i++) {
      if (this.data[i][key] === data[key]) {
        this.data[i][key] = data[key];
        return;
      }
    }
  }

  /**
   * @param {string} input
   */
  search(input) {
    if (input === "") {
      this.setData(this.data, false);
      return;
    }
    const columns = Array.from(this.headers.entries())
      .filter((header) => header[1].searchable)
      .map((header) => header[0]);
    const data = this.data.filter(
      (data) =>
        columns
          .map((column) =>
            data[column]
              .toLocaleLowerCase()
              .includes(input.toLocaleLowerCase()),
          )
          .filter((b) => b).length > 0,
    );
    this.setData(data, false);
  }
}

export { Header, LpmngTable };
