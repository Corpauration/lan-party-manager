class Device {
  /** @type string */
  id;
  /** @type string */
  mac;
  /** @type string */
  userId;
  /** @type boolean */
  internet;
  /** @type Date */
  dateTime;

  /**
   * @param {string} id
   * @param {string} mac
   * @param {string} userId
   * @param {boolean} internet
   * @param {Date} dateTime
   */
  constructor(id, mac, userId, internet, dateTime) {
    this.id = id;
    this.mac = mac;
    this.userId = userId;
    this.internet = internet;
    this.dateTime = dateTime;
  }

  /**
   * @param {{"id": string, "mac": string, "user_id": string, "internet": boolean, "date_time": string}} json
   * @return {Device}
   */
  static fromJson(json) {
    return new Device(
      json.id,
      json.mac,
      json.user_id,
      json.internet,
      new Date(json.date_time),
    );
  }
}

class DeviceInput {
  /** @type string */
  userId;

  /**
   * @param {string} userId
   */
  constructor(userId) {
    this.userId = userId;
  }

  /**
   * @returns {{user_id: string}}
   */
  toJson() {
    return {
      user_id: this.userId,
    };
  }
}

export { Device, DeviceInput };
