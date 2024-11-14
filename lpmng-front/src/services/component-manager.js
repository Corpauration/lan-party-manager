class ComponentManager {
  /** @type {Map.<string, Component>} */
  #map = new Map();

  /** @type {Map.<string, Array.<(Component) => void>>} */
  #listeners = new Map();

  constructor() {}

  /**
   * @param {string} id
   * @param {Component} component
   */
  register(id, component) {
    this.#map.set(id, component);
    if (this.#listeners.get(id) != null) {
      this.#listeners.get(id).forEach((f) => f(component));
    }
  }

  /**
   * @param {string} id
   * @param {(Component) => void} callback
   */
  listenTo(id, callback) {
    const listeners = this.#listeners.get(id) || [];
    listeners.push(callback);
    this.#listeners.set(id, listeners);
    if (this.#map.has(id)) {
      callback(this.#map.get(id));
    }
  }
}

const componentManager = new ComponentManager();

export { componentManager };
