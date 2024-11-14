const ResultType = {
  Ok: 0,
  Err: 1,
};

/** @template O
 * @template E */
class Result {
  /** @type ResultType */
  #type;
  /** @type {O|E} */
  #res;

  /**
   * @param {ResultType} type
   * @param {O|E} res
   */
  constructor(type, res) {
    this.#type = type;
    this.#res = res;
  }

  /** @returns {boolean} */
  isOk() {
    return this.#type === ResultType.Ok;
  }

  /** @returns {boolean} */
  isErr() {
    return this.#type === ResultType.Err;
  }

  /** @returns {?O} */
  ok() {
    return (this.#type === ResultType.Ok && this.#res) || null;
  }

  /** @returns {?E} */
  err() {
    return (this.#type === ResultType.Err && this.#res) || null;
  }
}

/** @template O */
class Ok extends Result {
  /**
   * @param {O} res
   */
  constructor(res) {
    super(ResultType.Ok, res);
  }
}

/** @template E */
class Err extends Result {
  /**
   * @param {E} res
   */
  constructor(res) {
    super(ResultType.Err, res);
  }
}

export { Result, Ok, Err };
