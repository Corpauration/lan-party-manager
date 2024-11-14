class Route {
  /** @type string */
  path;
  /** @type string */
  name;
  /** @type string */
  element;

  /**
   * @param {string} path
   * @param {string} name
   * @param {string} element
   */
  constructor(path, name, element) {
    this.path = path;
    this.name = name;
    this.element = element;
  }
}

class Router {
  /** @type Route[] */
  routes;
  /** @type Route */
  #route;
  /** @type Component */
  #element;
  /** @type {((Router) => void)[]} */
  #listeners = [];

  /**
   * @param {Route[]} routes
   */
  constructor(routes) {
    this.routes = routes;

    const redirect = sessionStorage.getItem("redirect");
    this.#route = this.routes.find((route) => route.path === (redirect || "/"));
    this.#createElement();
    history.pushState(null, "", redirect);
    sessionStorage.removeItem("redirect");
  }

  #createElement() {
    this.#element = document.createElement(this.#route.element);
  }

  /**
   * @param {function(Router): void} callback
   */
  addListener(callback) {
    this.#listeners.push(callback);
  }

  /**
   * @param {string} path
   */
  navigateTo(path) {
    path = path.replace(document.location.origin, "");
    const route = this.routes.find((route) => route.path === path);
    if (path == null) {
      throw "route not found";
    }

    this.#route = route;
    history.pushState(null, "", path);
    this.#createElement();
    this.#listeners.forEach((f) => f(this));
  }

  /**
   * @returns {string}
   */
  get path() {
    return this.#route.path;
  }

  /**
   * @returns {string}
   */
  get name() {
    return this.#route.name;
  }

  /**
   * @returns {Component}
   */
  get element() {
    return this.#element;
  }
}

const router = new Router([
  new Route("/", "LPMNG", "main-page"),
  new Route("/login", "Connexion", "login-page"),
  new Route("/register", "Inscription", "register-page"),
  new Route("/no-internet", "Pas d'internet :/", "no-internet-page"),
  new Route("/admin", "Admin", "admin-page"),
]);
window.addEventListener("popstate", () =>
  router.navigateTo(window.location.pathname),
);

export { Route, router };
