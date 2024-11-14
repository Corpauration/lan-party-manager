export class Component extends HTMLElement {
  /** @type string */
  inesId;

  /**
   * @param {!string} id
   */
  constructor(id) {
    super();
    this.inesId = id;

    let template = document.getElementById(this.inesId);
    let templateContent = template.content;

    const shadowRoot = this.attachShadow({ mode: "open" });
    shadowRoot.appendChild(templateContent.cloneNode(true));
  }
}
