/**
 * PDF Object Model
 *
 * This implementation mirrors the Rust `pdf::object` module for 100% API compatibility.
 */

import { PdfObjectType } from '../types.js';

/**
 * Base class for all PDF objects
 */
export abstract class PdfObject {
  abstract get type(): PdfObjectType;

  get isNull(): boolean { return this.type === PdfObjectType.Null; }
  get isBool(): boolean { return this.type === PdfObjectType.Bool; }
  get isInt(): boolean { return this.type === PdfObjectType.Int; }
  get isReal(): boolean { return this.type === PdfObjectType.Real; }
  get isNumber(): boolean { return this.isInt || this.isReal; }
  get isString(): boolean { return this.type === PdfObjectType.String; }
  get isName(): boolean { return this.type === PdfObjectType.Name; }
  get isArray(): boolean { return this.type === PdfObjectType.Array; }
  get isDict(): boolean { return this.type === PdfObjectType.Dict; }
  get isStream(): boolean { return this.type === PdfObjectType.Stream; }
  get isIndirect(): boolean { return this.type === PdfObjectType.Indirect; }

  toBool(): boolean {
    if (this instanceof PdfBool) return this.value;
    return false;
  }

  toInt(): number {
    if (this instanceof PdfInt) return this.value;
    if (this instanceof PdfReal) return Math.trunc(this.value);
    return 0;
  }

  toReal(): number {
    if (this instanceof PdfReal) return this.value;
    if (this instanceof PdfInt) return this.value;
    return 0;
  }

  toNumber(): number {
    return this.toReal();
  }

  toString(): string {
    if (this instanceof PdfString) return this.value;
    return '';
  }

  toName(): string {
    if (this instanceof PdfName) return this.value;
    return '';
  }

  nameEquals(name: string): boolean {
    if (this instanceof PdfName) return this.value === name;
    return false;
  }

  abstract equals(other: PdfObject): boolean;
}

/**
 * PDF Null object
 */
class PdfNull extends PdfObject {
  get type(): PdfObjectType { return PdfObjectType.Null; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfNull;
  }
}

/**
 * PDF Boolean object
 */
class PdfBool extends PdfObject {
  readonly value: boolean;

  constructor(value: boolean) {
    super();
    this.value = value;
  }

  get type(): PdfObjectType { return PdfObjectType.Bool; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfBool && other.value === this.value;
  }
}

/**
 * PDF Integer object
 */
class PdfInt extends PdfObject {
  readonly value: number;

  constructor(value: number) {
    super();
    this.value = Math.trunc(value);
  }

  get type(): PdfObjectType { return PdfObjectType.Int; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfInt && other.value === this.value;
  }
}

/**
 * PDF Real (floating point) object
 */
class PdfReal extends PdfObject {
  readonly value: number;

  constructor(value: number) {
    super();
    this.value = value;
  }

  get type(): PdfObjectType { return PdfObjectType.Real; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfReal && other.value === this.value;
  }
}

/**
 * PDF String object
 */
class PdfString extends PdfObject {
  readonly value: string;

  constructor(value: string) {
    super();
    this.value = value;
  }

  get type(): PdfObjectType { return PdfObjectType.String; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfString && other.value === this.value;
  }
}

/**
 * PDF Name object
 */
class PdfName extends PdfObject {
  readonly value: string;

  constructor(value: string) {
    super();
    this.value = value;
  }

  get type(): PdfObjectType { return PdfObjectType.Name; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfName && other.value === this.value;
  }
}

/**
 * PDF Array object
 */
export class PdfArray extends PdfObject {
  private readonly items: PdfObject[];
  private dirty: boolean = false;

  constructor(items: PdfObject[] = []) {
    super();
    this.items = [...items];
  }

  get type(): PdfObjectType { return PdfObjectType.Array; }

  /**
   * Get the number of elements in the array
   */
  get length(): number {
    return this.items.length;
  }

  /**
   * Get element at the specified index
   */
  get(index: number): PdfObject | undefined {
    return this.items[index];
  }

  /**
   * Set element at the specified index (alias for put)
   */
  put(index: number, obj: PdfObject): void {
    if (index >= 0 && index < this.items.length) {
      this.items[index] = obj;
      this.dirty = true;
    }
  }

  /**
   * Push a PDF object to the end of the array
   */
  push(obj: PdfObject): void {
    this.items.push(obj);
    this.dirty = true;
  }

  /**
   * Push an integer value to the end of the array
   */
  pushInt(value: number): void {
    this.items.push(new PdfInt(value));
    this.dirty = true;
  }

  /**
   * Push a real (float) value to the end of the array
   */
  pushReal(value: number): void {
    this.items.push(new PdfReal(value));
    this.dirty = true;
  }

  /**
   * Push a boolean value to the end of the array
   */
  pushBool(value: boolean): void {
    this.items.push(new PdfBool(value));
    this.dirty = true;
  }

  /**
   * Push a name to the end of the array
   */
  pushName(value: string): void {
    this.items.push(new PdfName(value));
    this.dirty = true;
  }

  /**
   * Push a string to the end of the array
   */
  pushString(value: string): void {
    this.items.push(new PdfString(value));
    this.dirty = true;
  }

  /**
   * Insert element at the specified index
   */
  insert(index: number, obj: PdfObject): void {
    if (index >= 0 && index <= this.items.length) {
      this.items.splice(index, 0, obj);
      this.dirty = true;
    }
  }

  /**
   * Delete element at the specified index
   */
  delete(index: number): void {
    if (index >= 0 && index < this.items.length) {
      this.items.splice(index, 1);
      this.dirty = true;
    }
  }

  /**
   * Convert to a JavaScript array
   */
  toArray(): PdfObject[] {
    return [...this.items];
  }

  /**
   * Check if this array has been modified
   */
  isDirty(): boolean {
    return this.dirty;
  }

  /**
   * Mark this array as dirty (modified)
   */
  markDirty(): void {
    this.dirty = true;
  }

  /**
   * Mark this array as clean (not modified)
   */
  markClean(): void {
    this.dirty = false;
  }

  /**
   * Create a shallow copy of this array
   */
  copy(): PdfArray {
    return new PdfArray([...this.items]);
  }

  /**
   * Create a deep copy of this array
   */
  deepCopy(): PdfArray {
    const copied = this.items.map(item => {
      if (item instanceof PdfArray) return item.deepCopy();
      if (item instanceof PdfDict) return item.deepCopy();
      if (item instanceof PdfStream) return item.deepCopy();
      return item; // Primitive types are immutable
    });
    return new PdfArray(copied);
  }

  [Symbol.iterator](): Iterator<PdfObject> {
    return this.items[Symbol.iterator]();
  }

  equals(other: PdfObject): boolean {
    if (!(other instanceof PdfArray)) return false;
    if (other.length !== this.length) return false;
    for (let i = 0; i < this.length; i++) {
      if (!this.items[i]!.equals(other.items[i]!)) return false;
    }
    return true;
  }
}

/**
 * PDF Dictionary object
 */
export class PdfDict extends PdfObject {
  private readonly entries: Map<string, PdfObject>;
  private dirty: boolean = false;

  constructor(entries: Record<string, PdfObject> = {}) {
    super();
    this.entries = new Map(Object.entries(entries));
  }

  get type(): PdfObjectType { return PdfObjectType.Dict; }

  /**
   * Get the number of entries in the dictionary
   */
  get length(): number {
    return this.entries.size;
  }

  /**
   * Get value for a key
   */
  get(key: string): PdfObject | undefined {
    return this.entries.get(key);
  }

  /**
   * Put a PDF object value for a key
   */
  put(key: string, value: PdfObject): void {
    this.entries.set(key, value);
    this.dirty = true;
  }

  /**
   * Put an integer value for a key
   */
  putInt(key: string, value: number): void {
    this.entries.set(key, new PdfInt(value));
    this.dirty = true;
  }

  /**
   * Put a real (float) value for a key
   */
  putReal(key: string, value: number): void {
    this.entries.set(key, new PdfReal(value));
    this.dirty = true;
  }

  /**
   * Put a boolean value for a key
   */
  putBool(key: string, value: boolean): void {
    this.entries.set(key, new PdfBool(value));
    this.dirty = true;
  }

  /**
   * Put a name for a key
   */
  putName(key: string, value: string): void {
    this.entries.set(key, new PdfName(value));
    this.dirty = true;
  }

  /**
   * Put a string for a key
   */
  putString(key: string, value: string): void {
    this.entries.set(key, new PdfString(value));
    this.dirty = true;
  }

  /**
   * Delete a key from the dictionary
   */
  del(key: string): void {
    if (this.entries.delete(key)) {
      this.dirty = true;
    }
  }

  /**
   * Check if a key exists in the dictionary
   */
  has(key: string): boolean {
    return this.entries.has(key);
  }

  /**
   * Get all keys in the dictionary
   */
  keys(): string[] {
    return Array.from(this.entries.keys());
  }

  /**
   * Get all values in the dictionary
   */
  values(): PdfObject[] {
    return Array.from(this.entries.values());
  }

  /**
   * Get all entries as [key, value] pairs
   */
  entries(): [string, PdfObject][] {
    return Array.from(this.entries.entries());
  }

  *[Symbol.iterator](): Generator<[string, PdfObject]> {
    for (const [key, value] of this.entries) {
      yield [key, value];
    }
  }

  // ============================================================================
  // Typed getters with defaults
  // ============================================================================

  /**
   * Get name value for a key
   */
  getName(key: string): string {
    return this.get(key)?.toName() ?? '';
  }

  /**
   * Get string value for a key
   */
  getString(key: string): string {
    return this.get(key)?.toString() ?? '';
  }

  /**
   * Get integer value for a key
   */
  getInt(key: string, defaultValue: number = 0): number {
    const obj = this.get(key);
    return obj ? obj.toInt() : defaultValue;
  }

  /**
   * Get real (float) value for a key
   */
  getReal(key: string, defaultValue: number = 0): number {
    const obj = this.get(key);
    return obj ? obj.toReal() : defaultValue;
  }

  /**
   * Get boolean value for a key
   */
  getBool(key: string, defaultValue: boolean = false): boolean {
    const obj = this.get(key);
    return obj ? obj.toBool() : defaultValue;
  }

  /**
   * Get array for a key
   */
  getArray(key: string): PdfArray | undefined {
    const obj = this.get(key);
    return obj instanceof PdfArray ? obj : undefined;
  }

  /**
   * Get dictionary for a key
   */
  getDict(key: string): PdfDict | undefined {
    const obj = this.get(key);
    return obj instanceof PdfDict ? obj : undefined;
  }

  /**
   * Get stream for a key
   */
  getStream(key: string): PdfStream | undefined {
    const obj = this.get(key);
    return obj instanceof PdfStream ? obj : undefined;
  }

  // ============================================================================
  // Dirty tracking
  // ============================================================================

  /**
   * Check if this dictionary has been modified
   */
  isDirty(): boolean {
    return this.dirty;
  }

  /**
   * Mark this dictionary as dirty (modified)
   */
  markDirty(): void {
    this.dirty = true;
  }

  /**
   * Mark this dictionary as clean (not modified)
   */
  markClean(): void {
    this.dirty = false;
  }

  // ============================================================================
  // Copying
  // ============================================================================

  /**
   * Create a shallow copy of this dictionary
   */
  copy(): PdfDict {
    const entries: Record<string, PdfObject> = {};
    for (const [key, value] of this.entries) {
      entries[key] = value;
    }
    return new PdfDict(entries);
  }

  /**
   * Create a deep copy of this dictionary
   */
  deepCopy(): PdfDict {
    const entries: Record<string, PdfObject> = {};
    for (const [key, value] of this.entries) {
      if (value instanceof PdfArray) {
        entries[key] = value.deepCopy();
      } else if (value instanceof PdfDict) {
        entries[key] = value.deepCopy();
      } else if (value instanceof PdfStream) {
        entries[key] = value.deepCopy();
      } else {
        entries[key] = value; // Primitive types are immutable
      }
    }
    return new PdfDict(entries);
  }

  equals(other: PdfObject): boolean {
    if (!(other instanceof PdfDict)) return false;
    if (other.length !== this.length) return false;
    for (const [key, value] of this.entries) {
      const otherValue = other.get(key);
      if (!otherValue || !value.equals(otherValue)) return false;
    }
    return true;
  }
}

/**
 * PDF Stream object
 */
export class PdfStream extends PdfObject {
  readonly dict: PdfDict;
  private _data: Uint8Array;

  constructor(dict: PdfDict, data: Uint8Array = new Uint8Array(0)) {
    super();
    this.dict = dict;
    this._data = data;
  }

  get type(): PdfObjectType { return PdfObjectType.Stream; }

  /**
   * Get the stream data
   */
  getData(): Uint8Array {
    return this._data;
  }

  /**
   * Set the stream data
   */
  setData(data: Uint8Array): void {
    this._data = data;
    this.dict.markDirty();
  }

  /**
   * Get the stream dictionary
   */
  getDict(): PdfDict {
    return this.dict;
  }

  /**
   * Create a deep copy of this stream
   */
  deepCopy(): PdfStream {
    const dictCopy = this.dict.deepCopy();
    const dataCopy = new Uint8Array(this._data);
    return new PdfStream(dictCopy, dataCopy);
  }

  equals(other: PdfObject): boolean {
    if (!(other instanceof PdfStream)) return false;
    if (!this.dict.equals(other.dict)) return false;
    if (this._data.length !== other._data.length) return false;
    for (let i = 0; i < this._data.length; i++) {
      if (this._data[i] !== other._data[i]) return false;
    }
    return true;
  }
}

/**
 * PDF Indirect Reference object
 */
export class PdfIndirectRef extends PdfObject {
  readonly objNum: number;
  readonly genNum: number;

  constructor(objNum: number, genNum: number = 0) {
    super();
    this.objNum = objNum;
    this.genNum = genNum;
  }

  get type(): PdfObjectType { return PdfObjectType.Indirect; }

  equals(other: PdfObject): boolean {
    return other instanceof PdfIndirectRef &&
           other.objNum === this.objNum &&
           other.genNum === this.genNum;
  }
}

// ============================================================================
// Factory Functions
// ============================================================================

const NULL_SINGLETON = new PdfNull();

/**
 * Create a null PDF object
 */
export function pdfNull(): PdfObject {
  return NULL_SINGLETON;
}

/**
 * Create a boolean PDF object
 */
export function pdfBool(value: boolean): PdfObject {
  return new PdfBool(value);
}

/**
 * Create an integer PDF object
 */
export function pdfInt(value: number): PdfObject {
  return new PdfInt(value);
}

/**
 * Create a real PDF object
 */
export function pdfReal(value: number): PdfObject {
  return new PdfReal(value);
}

/**
 * Create a string PDF object
 */
export function pdfString(value: string): PdfObject {
  return new PdfString(value);
}

/**
 * Create a name PDF object
 */
export function pdfName(value: string): PdfObject {
  return new PdfName(value);
}

/**
 * Create an array PDF object
 */
export function pdfArray(items: PdfObject[] = []): PdfArray {
  return new PdfArray(items);
}

/**
 * Create a dictionary PDF object
 */
export function pdfDict(entries: Record<string, PdfObject> = {}): PdfDict {
  return new PdfDict(entries);
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Compare two PDF objects for equality
 */
export function pdfObjectCompare(a: PdfObject, b: PdfObject): boolean {
  return a.equals(b);
}

/**
 * Check if a PDF name equals a string value
 */
export function pdfNameEquals(obj: PdfObject, name: string): boolean {
  return obj.nameEquals(name);
}

/**
 * Create a deep copy of a PDF object
 */
export function pdfDeepCopy(obj: PdfObject): PdfObject {
  if (obj instanceof PdfArray) return obj.deepCopy();
  if (obj instanceof PdfDict) return obj.deepCopy();
  if (obj instanceof PdfStream) return obj.deepCopy();
  // Primitive types are immutable, so return as-is
  return obj;
}

/**
 * Create a shallow copy of an array
 */
export function pdfCopyArray(array: PdfArray): PdfArray {
  return array.copy();
}

/**
 * Create a shallow copy of a dictionary
 */
export function pdfCopyDict(dict: PdfDict): PdfDict {
  return dict.copy();
}

// ============================================================================
// Type Checking Functions
// ============================================================================

/**
 * Check if object is null
 */
export function isNull(obj: PdfObject): boolean {
  return obj.isNull;
}

/**
 * Check if object is a boolean
 */
export function isBool(obj: PdfObject): boolean {
  return obj.isBool;
}

/**
 * Check if object is an integer
 */
export function isInt(obj: PdfObject): boolean {
  return obj.isInt;
}

/**
 * Check if object is a real number
 */
export function isReal(obj: PdfObject): boolean {
  return obj.isReal;
}

/**
 * Check if object is a number (int or real)
 */
export function isNumber(obj: PdfObject): boolean {
  return obj.isNumber;
}

/**
 * Check if object is a name
 */
export function isName(obj: PdfObject): boolean {
  return obj.isName;
}

/**
 * Check if object is a string
 */
export function isString(obj: PdfObject): boolean {
  return obj.isString;
}

/**
 * Check if object is an array
 */
export function isArray(obj: PdfObject): boolean {
  return obj.isArray;
}

/**
 * Check if object is a dictionary
 */
export function isDict(obj: PdfObject): boolean {
  return obj.isDict;
}

/**
 * Check if object is a stream
 */
export function isStream(obj: PdfObject): boolean {
  return obj.isStream;
}

/**
 * Check if object is an indirect reference
 */
export function isIndirect(obj: PdfObject): boolean {
  return obj.isIndirect;
}

// ============================================================================
// Value Extraction Functions (with defaults)
// ============================================================================

/**
 * Convert object to boolean with default value
 */
export function toBoolDefault(obj: PdfObject | undefined, defaultValue: boolean): boolean {
  return obj ? obj.toBool() : defaultValue;
}

/**
 * Convert object to integer with default value
 */
export function toIntDefault(obj: PdfObject | undefined, defaultValue: number): number {
  return obj ? obj.toInt() : defaultValue;
}

/**
 * Convert object to real with default value
 */
export function toRealDefault(obj: PdfObject | undefined, defaultValue: number): number {
  return obj ? obj.toReal() : defaultValue;
}

/**
 * Get object number from indirect reference
 */
export function toObjNum(obj: PdfObject): number {
  if (obj instanceof PdfIndirectRef) {
    return obj.objNum;
  }
  return 0;
}

/**
 * Get generation number from indirect reference
 */
export function toGenNum(obj: PdfObject): number {
  if (obj instanceof PdfIndirectRef) {
    return obj.genNum;
  }
  return 0;
}

