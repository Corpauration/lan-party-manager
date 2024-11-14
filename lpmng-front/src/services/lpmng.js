import { Credentials, Login } from "../models/login.js";
import { UserView } from "../models/user.js";
import { Device, DeviceInput } from "../models/device.js";
import { router } from "./router.js";

const base = "http://localhost/api";

class Lpmng {
  /** @type ?Credentials */
  creds;

  constructor() {
    const creds = localStorage.getItem("creds");
    if (creds != null) {
      const tmp = JSON.parse(creds);
      this.creds = new Credentials(tmp.biscuit, tmp.role, tmp.userId);
      this.updateAuthState().then(() => console.log("auth state updated"));
    }
  }

  /**
   * @param {Login} login
   * @returns {Promise<void>}
   */
  async login(login) {
    const res = await fetch(`${base}/login`, {
      body: JSON.stringify(login.toJson()),
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Request-Method": "POST",
      },
      method: "POST",
    });

    if (!res.ok) {
      throw await res.text();
    }

    const creds = Credentials.fromJson(await res.json());
    this.creds = creds;
    localStorage.setItem("creds", JSON.stringify(creds));
  }

  logout() {
    localStorage.removeItem("creds");
    this.creds = null;
    router.navigateTo("/login");
  }

  /**
   * @param {UserInput} user
   * @returns {Promise<void>}
   */
  async register(user) {
    if (this.creds != null) {
      throw "déjà connecté";
    }

    const res = await fetch(`${base}/users`, {
      body: JSON.stringify(user),
      headers: {
        "Content-Type": "application/json",
        "Access-Control-Request-Method": "POST",
      },
      method: "POST",
    });

    if (!res.ok) {
      throw await res.text();
    }

    await this.login(new Login(user.username, user.password));
  }

  async addDevice() {
    if (this.creds == null) {
      throw "pas connecté";
    }

    const device = new DeviceInput(this.creds.userId);
    const res = await fetch(`${base}/devices`, {
      body: JSON.stringify(device.toJson()),
      headers: {
        Authorization: `Bearer ${this.creds.biscuit}`,
        "Content-Type": "application/json",
        "Access-Control-Request-Method": "POST",
      },
      method: "POST",
    });

    if (!res.ok) {
      throw await res.text();
    }
  }

  /**
   * @param {string} id
   * @return {Promise<UserView>}
   */
  async getUser(id) {
    if (this.creds == null) {
      throw "pas connecté";
    } else if (this.creds.userId !== id && this.creds.role !== "admin") {
      throw "non autorisé";
    }

    const res = await fetch(`${base}/users/${id}`, {
      headers: {
        Authorization: `Bearer ${this.creds.biscuit}`,
        "Access-Control-Request-Method": "POST",
      },
    });

    if (!res.ok) {
      throw await res.text();
    }

    return UserView.fromJson(await res.json());
  }

  async updateAuthState() {
    if (this.creds == null) {
      return;
    }

    try {
      if (this.creds.userId !== "00000000-0000-0000-0000-000000000000") {
        const user = await this.getUser(this.creds.userId);
        this.creds.role = user.role;
      }
    } catch {
      this.logout();
    }
  }

  /**
   * @param {string} id
   * @return {Promise<Device[]>}
   */
  async getUserDevices(id) {
    if (this.creds == null) {
      throw "pas connecté";
    } else if (this.creds.userId !== id && this.creds.role !== "admin") {
      throw "non autorisé";
    }

    const res = await fetch(`${base}/devices/${id}`, {
      headers: {
        Authorization: `Bearer ${this.creds.biscuit}`,
        "Access-Control-Request-Method": "POST",
      },
    });

    if (!res.ok) {
      throw await res.text();
    }

    /** @type any[] */
    const list = await res.json();

    return list.map((e) => Device.fromJson(e));
  }

  /**
   * @return {Promise<UserView[]>}
   */
  async getUsers() {
    if (this.creds == null) {
      throw "pas connecté";
    } else if (this.creds.role !== "admin") {
      throw "non autorisé";
    }

    const res = await fetch(`${base}/users`, {
      headers: {
        Authorization: `Bearer ${this.creds.biscuit}`,
        "Access-Control-Request-Method": "GET",
      },
    });

    if (!res.ok) {
      throw await res.text();
    }

    /** @type any[] */
    const list = await res.json();

    return list.map((e) => UserView.fromJson(e));
  }

  /**
   * @param {string} id
   * @param {boolean} bool
   * @returns {Promise<void>}
   */
  async setInternet(id, bool) {
    if (this.creds == null) {
      throw "pas connecté";
    } else if (this.creds.role !== "admin") {
      throw "non autorisé";
    }

    const res = await fetch(`${base}/users`, {
      body: JSON.stringify({ id: id, is_allowed: bool }),
      headers: {
        Authorization: `Bearer ${this.creds.biscuit}`,
        "Content-Type": "application/json",
        "Access-Control-Request-Method": "PATCH",
      },
      method: "PATCH",
    });

    if (!res.ok) {
      throw await res.text();
    }
  }
}

const lpmng = new Lpmng();

export { lpmng };
